use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::engine::config::EngineConfig;
use crate::engine::timing::{calculate_delay, calculate_delay_with_unfamiliar, punctuation_pause_with_config, is_unfamiliar_word};
use crate::engine::pauses::{
    self, is_sentence_end, is_clause_boundary, paragraph_pause, should_think, thinking_pause,
    preprocess_text, TextSegment,
};
use crate::engine::errors::{ErrorSystem, ErrorAction, CorrectionStep};
use crate::engine::behaviors::{
    FatigueTracker, MicroCorrectionTracker, SecondThoughtsTracker, BurstPauseTracker,
    WordSubstitutionTracker, HesitationBackspaceTracker, MidWordPauseTracker, SentenceRestartTracker,
    FatiguePhase, should_rollover,
};
use crate::engine::events::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineState {
    Idle,
    Running,
    Paused,
    Completed,
    Cancelled,
}

impl std::fmt::Display for EngineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineState::Idle => write!(f, "idle"),
            EngineState::Running => write!(f, "running"),
            EngineState::Paused => write!(f, "paused"),
            EngineState::Completed => write!(f, "completed"),
            EngineState::Cancelled => write!(f, "cancelled"),
        }
    }
}

pub type SharedState = Arc<Mutex<EngineState>>;

pub enum InjectorCommand {
    TypeChar(char),
    Backspace,
    Stop,
}

pub fn new_shared_state() -> SharedState {
    Arc::new(Mutex::new(EngineState::Idle))
}

pub struct TypingStats {
    pub total_chars: usize,
    pub total_errors: usize,
    pub total_corrections: usize,
    pub elapsed_ms: u64,
    pub average_wpm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateResult {
    pub total_seconds: f64,
    pub typing_seconds: f64,
    pub pause_seconds: f64,
    pub estimated_errors: usize,
}

/// Estimate how long it will take to type the given text with the given config.
pub fn estimate_typing_time(text: &str, config: &EngineConfig) -> EstimateResult {
    let segments = preprocess_text(text);
    let total_chars: usize = segments.iter().map(|s| match s {
        TextSegment::Text(t) => t.len(),
        _ => 0,
    }).sum();

    // Base typing time from WPM
    let chars_per_second = (config.wpm * 5.0) / 60.0;
    let typing_seconds = total_chars as f64 / chars_per_second;

    // Estimate pause time
    let mut pause_seconds = 0.0;

    // Inline pauses
    for seg in &segments {
        match seg {
            TextSegment::Pause(d) => pause_seconds += d.as_secs_f64(),
            TextSegment::Think => {
                let (min, max) = config.thinking_pause.preset.range_ms();
                pause_seconds += (min + max) as f64 / 2000.0;
            }
            TextSegment::LongPause => {
                let (min, max) = config.paragraph_pause.range_ms();
                pause_seconds += (min + max) as f64 / 2000.0;
            }
            TextSegment::Text(t) => {
                // Count sentence ends for thinking pauses
                let sentence_ends = t.chars().filter(|c| is_sentence_end(*c)).count();
                if config.thinking_pause.enabled {
                    let avg_thinking = sentence_ends as f64 * config.thinking_pause.frequency;
                    let (min, max) = config.thinking_pause.preset.range_ms();
                    pause_seconds += avg_thinking * (min + max) as f64 / 2000.0;
                }

                // Count paragraph breaks
                let paragraphs = t.matches("\n\n").count();
                let (pmin, pmax) = config.paragraph_pause.range_ms();
                pause_seconds += paragraphs as f64 * (pmin + pmax) as f64 / 2000.0;
            }
        }
    }

    let estimated_errors = (total_chars as f64 * config.error_config.error_rate) as usize;
    let total_seconds = typing_seconds + pause_seconds;

    EstimateResult {
        total_seconds,
        typing_seconds,
        pause_seconds,
        estimated_errors,
    }
}

/// Helper: extract the current word being built from recent text
fn current_word_from_text(recent_text: &str) -> String {
    recent_text
        .rsplit(|c: char| c.is_whitespace() || is_sentence_end(c))
        .next()
        .unwrap_or("")
        .to_string()
}

/// Helper: extract the last complete sentence from recent text
fn last_sentence_from_text(recent_text: &str) -> Option<String> {
    // Find the second-to-last sentence end
    let trimmed = recent_text.trim_end();
    if trimmed.is_empty() {
        return None;
    }

    // Find where the last sentence started (after the previous sentence-end punctuation)
    let chars: Vec<char> = trimmed.chars().collect();
    let mut end = chars.len();
    // Walk backwards to find the sentence-ending char
    if end > 0 && is_sentence_end(chars[end - 1]) {
        end -= 1; // skip the trailing punctuation itself
    }

    // Now walk backwards to find the START of this sentence (previous sentence end or beginning)
    let mut start = 0;
    for j in (0..end).rev() {
        if is_sentence_end(chars[j]) {
            start = j + 1;
            break;
        }
    }

    // Trim leading whitespace
    while start < chars.len() && chars[start].is_whitespace() {
        start += 1;
    }

    if start >= end {
        return None;
    }

    let sentence: String = chars[start..=end].iter().collect();
    if sentence.split_whitespace().count() >= 3 {
        Some(sentence)
    } else {
        None
    }
}

/// Helper to type a char and emit events
async fn type_char_helper(
    c: char,
    delay_ms: u64,
    position: usize,
    is_correct: bool,
    injector_tx: &Option<mpsc::Sender<InjectorCommand>>,
    app_handle: &AppHandle,
    state: &SharedState,
) {
    let _ = app_handle.emit("typing-char", TypingCharEvent {
        position, char_typed: c, delay_ms, is_correct,
    });
    if let Some(ref tx) = injector_tx {
        let _ = tx.send(InjectorCommand::TypeChar(c));
    }
    async_sleep_with_state(Duration::from_millis(delay_ms), state).await;
}

/// Helper to backspace and emit events
async fn backspace_helper(
    delay_ms: u64,
    position: usize,
    injector_tx: &Option<mpsc::Sender<InjectorCommand>>,
    app_handle: &AppHandle,
    state: &SharedState,
) {
    let _ = app_handle.emit("typing-char", TypingCharEvent {
        position, char_typed: '\x08', delay_ms, is_correct: true,
    });
    if let Some(ref tx) = injector_tx {
        let _ = tx.send(InjectorCommand::Backspace);
    }
    async_sleep_with_state(Duration::from_millis(delay_ms), state).await;
}

/// The main typing engine. Processes text and emits events for each action.
pub async fn run_typing_session(
    text: String,
    config: EngineConfig,
    state: SharedState,
    app_handle: AppHandle,
    inject_keys: bool,
    countdown_seconds: u32,
) -> TypingStats {
    // Set up key injector thread if injection is enabled
    let injector_tx = if inject_keys {
        let (tx, rx) = mpsc::channel::<InjectorCommand>();
        std::thread::spawn(move || {
            use enigo::{Enigo, Keyboard, Key, Direction, Settings};
            let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    InjectorCommand::TypeChar(c) => {
                        match c {
                            '\n' | '\r' => { let _ = enigo.key(Key::Return, Direction::Click); }
                            '\t' => { let _ = enigo.key(Key::Tab, Direction::Click); }
                            ' ' => { let _ = enigo.key(Key::Space, Direction::Click); }
                            _ => { let _ = enigo.text(&c.to_string()); }
                        }
                    }
                    InjectorCommand::Backspace => {
                        let _ = enigo.key(Key::Backspace, Direction::Click);
                    }
                    InjectorCommand::Stop => break,
                }
            }
        });
        Some(tx)
    } else {
        None
    };

    // Countdown before typing starts
    if countdown_seconds > 0 {
        let _ = app_handle.emit("typing-state", TypingStateEvent { state: "countdown".to_string() });
        for i in (1..=countdown_seconds).rev() {
            let _ = app_handle.emit("typing-countdown", TypingCountdownEvent { seconds_left: i });
            tokio::time::sleep(Duration::from_secs(1)).await;
            let current = { *state.lock().unwrap() };
            if current == EngineState::Cancelled {
                if let Some(tx) = injector_tx {
                    let _ = tx.send(InjectorCommand::Stop);
                }
                return build_stats(0, 0, 0, Instant::now());
            }
        }
        let _ = app_handle.emit("typing-countdown", TypingCountdownEvent { seconds_left: 0 });
    }

    let mut rng = StdRng::from_entropy();
    let segments = preprocess_text(&text);
    let start_time = Instant::now();

    // Flatten text segments into chars for total count
    let total_chars: usize = segments.iter().map(|s| match s {
        TextSegment::Text(t) => t.len(),
        _ => 0,
    }).sum();

    let mut error_system = ErrorSystem::new();
    let fatigue_tracker = FatigueTracker::new(&config.fatigue_config);
    let mut micro_tracker = MicroCorrectionTracker::new();
    let mut second_thoughts_tracker = SecondThoughtsTracker::new();
    let mut burst_tracker = config.burst_pause.as_ref().map(|bp| BurstPauseTracker::new(bp, &mut rng));

    // New behavior trackers
    let mut word_sub_tracker = WordSubstitutionTracker::new();
    let mut hesitation_bs_tracker = HesitationBackspaceTracker::new();
    let mut mid_word_tracker = MidWordPauseTracker::new();
    let mut sentence_restart_tracker = SentenceRestartTracker::new();
    let mut last_auto_pause_time = Instant::now();

    let mut position: usize = 0;
    let mut errors_made: usize = 0;
    let mut corrections_made: usize = 0;
    let mut recent_chars: Vec<char> = Vec::new();
    let mut recent_text = String::new();
    let mut current_word = String::new();
    let mut in_unfamiliar_word = false;

    // Set state to running
    {
        let mut s = state.lock().unwrap();
        *s = EngineState::Running;
    }
    let _ = app_handle.emit("typing-state", TypingStateEvent { state: "running".to_string() });

    for segment in &segments {
        match segment {
            TextSegment::Pause(duration) => {
                let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                    position,
                    pause_type: "inline".to_string(),
                    duration_ms: duration.as_millis() as u64,
                });
                async_sleep_with_state(*duration, &state).await;
            }
            TextSegment::Think => {
                let duration = thinking_pause(&config.thinking_pause, &mut rng);
                let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                    position,
                    pause_type: "thinking".to_string(),
                    duration_ms: duration.as_millis() as u64,
                });
                async_sleep_with_state(duration, &state).await;
            }
            TextSegment::LongPause => {
                let duration = paragraph_pause(&config.paragraph_pause, &mut rng);
                let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                    position,
                    pause_type: "paragraph".to_string(),
                    duration_ms: duration.as_millis() as u64,
                });
                async_sleep_with_state(duration, &state).await;
            }
            TextSegment::Text(text_content) => {
                let chars: Vec<char> = text_content.chars().collect();
                let mut i = 0;
                let mut word_pos: usize = 0;

                while i < chars.len() {
                    // Check state
                    let current_state = {
                        let s = state.lock().unwrap();
                        *s
                    };
                    match current_state {
                        EngineState::Cancelled => {
                            let _ = app_handle.emit("typing-state", TypingStateEvent { state: "cancelled".to_string() });
                            if let Some(ref tx) = injector_tx {
                                let _ = tx.send(InjectorCommand::Stop);
                            }
                            return build_stats(total_chars, errors_made, corrections_made, start_time);
                        }
                        EngineState::Paused => {
                            loop {
                                tokio::time::sleep(Duration::from_millis(100)).await;
                                let s = state.lock().unwrap();
                                if *s != EngineState::Paused {
                                    break;
                                }
                            }
                            continue;
                        }
                        _ => {}
                    }

                    // Auto-pause check
                    if config.auto_pause.enabled {
                        let since_last = last_auto_pause_time.elapsed().as_secs_f64() / 60.0;
                        if since_last >= config.auto_pause.interval_minutes {
                            let pause_dur = Duration::from_secs_f64(config.auto_pause.duration_minutes * 60.0);
                            let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                position,
                                pause_type: "auto_pause".to_string(),
                                duration_ms: pause_dur.as_millis() as u64,
                            });
                            async_sleep_with_state(pause_dur, &state).await;
                            last_auto_pause_time = Instant::now();
                        }
                    }

                    let c = chars[i];
                    let prev_char = if i > 0 { Some(chars[i - 1]) } else { None };
                    let next_char = if i + 1 < chars.len() { Some(chars[i + 1]) } else { None };

                    // Track word boundaries
                    let is_word_boundary = c == ' ' || c == '\n';
                    if is_word_boundary {
                        // Word just ended — check unfamiliar and update trackers
                        word_sub_tracker.on_word_boundary();
                        second_thoughts_tracker.on_word_end();

                        // Check for sentence restart at sentence ends
                        if is_sentence_end(c) || (prev_char.map_or(false, is_sentence_end)) {
                            sentence_restart_tracker.on_sentence_end();
                        }

                        word_pos = 0;
                        current_word.clear();
                        in_unfamiliar_word = false;
                    } else {
                        current_word.push(c);
                        word_pos += 1;

                        // Evaluate unfamiliar on first few chars of word (re-check as word grows)
                        if config.unfamiliar_word_slowdown && word_pos >= 3 {
                            in_unfamiliar_word = is_unfamiliar_word(&current_word);
                        }
                    }
                    recent_text.push(c);

                    // Get fatigue multipliers
                    let elapsed_min = start_time.elapsed().as_secs_f64() / 60.0;
                    let fatigue = fatigue_tracker.get_multipliers(elapsed_min, config.fatigue_config.enabled);

                    // --- HESITATION: Mid-word pause (only mid-word, not on boundaries) ---
                    if !is_word_boundary && word_pos > 1 {
                        mid_word_tracker.tick();
                        if mid_word_tracker.should_trigger(&config.mid_word_pause, &mut rng) {
                            let pause = mid_word_tracker.generate_pause(&config.mid_word_pause, &mut rng);
                            mid_word_tracker.reset();
                            let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                position,
                                pause_type: "mid_word".to_string(),
                                duration_ms: pause.as_millis() as u64,
                            });
                            async_sleep_with_state(pause, &state).await;
                        }
                    }

                    // --- HESITATION: Random backspace ---
                    hesitation_bs_tracker.tick();
                    if hesitation_bs_tracker.should_trigger(&config.hesitation_backspace, &mut rng) && recent_chars.len() >= 2 {
                        let action = hesitation_bs_tracker.generate_sequence(&recent_chars, &mut rng);
                        hesitation_bs_tracker.reset();

                        async_sleep_with_state(action.hesitation_pause, &state).await;
                        for _ in 0..action.backspace_count {
                            backspace_helper(action.backspace_speed.as_millis() as u64, position, &injector_tx, &app_handle, &state).await;
                        }
                        async_sleep_with_state(action.pre_retype_pause, &state).await;
                        for rc in &action.chars_to_retype {
                            type_char_helper(*rc, rng.gen_range(50..=90), position, true, &injector_tx, &app_handle, &state).await;
                        }
                    }

                    // --- Rule 15: Micro-correction check ---
                    micro_tracker.tick();
                    if micro_tracker.should_trigger(&config.micro_correction, &mut rng) && recent_chars.len() >= 3 {
                        let action = micro_tracker.generate_sequence(&recent_chars, &mut rng);
                        micro_tracker.reset();

                        async_sleep_with_state(action.notice_pause, &state).await;
                        for _ in 0..action.backspace_count {
                            backspace_helper(action.inter_backspace.as_millis() as u64, position, &injector_tx, &app_handle, &state).await;
                        }
                        async_sleep_with_state(action.pre_retype_pause, &state).await;
                        for rc in &action.chars_to_retype {
                            type_char_helper(*rc, rng.gen_range(50..=90), position, true, &injector_tx, &app_handle, &state).await;
                        }
                        corrections_made += 1;
                    }

                    // --- Rule 16: Second thoughts check (at word boundaries) ---
                    if (c == ' ' || is_sentence_end(c)) && second_thoughts_tracker.should_trigger(&config.second_thoughts, &mut rng) {
                        let action = second_thoughts_tracker.generate_sequence(&recent_text, &config.second_thoughts, &mut rng);
                        second_thoughts_tracker.reset();

                        for (original, replacement) in &action.synonyms_used {
                            let _ = app_handle.emit("typing-synonym", TypingSynonymEvent {
                                original: original.clone(),
                                replacement: replacement.clone(),
                            });
                        }

                        let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                            position,
                            pause_type: "second_thoughts".to_string(),
                            duration_ms: action.rethink_pause.as_millis() as u64,
                        });
                        async_sleep_with_state(action.rethink_pause, &state).await;

                        for _ in 0..action.chars_to_delete {
                            backspace_helper(action.delete_speed.as_millis() as u64, position, &injector_tx, &app_handle, &state).await;
                        }

                        async_sleep_with_state(action.reformulate_pause, &state).await;

                        for rc in action.text_to_retype.chars() {
                            let retype_delay = calculate_delay(rc, None, 0, &config, &mut rng);
                            type_char_helper(rc, retype_delay.as_millis() as u64, position, true, &injector_tx, &app_handle, &state).await;
                        }
                    }

                    // --- SENTENCE RESTART check (at sentence boundaries) ---
                    if is_sentence_end(c) && sentence_restart_tracker.should_trigger(&config.sentence_restart, &mut rng) {
                        if let Some(sentence) = last_sentence_from_text(&recent_text) {
                            let action = sentence_restart_tracker.generate_sequence(&sentence, &mut rng);
                            sentence_restart_tracker.reset();

                            let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                position,
                                pause_type: "sentence_restart".to_string(),
                                duration_ms: action.rethink_pause.as_millis() as u64,
                            });

                            // Delete the sentence
                            for _ in 0..action.chars_to_delete {
                                backspace_helper(action.delete_speed.as_millis() as u64, position, &injector_tx, &app_handle, &state).await;
                            }

                            async_sleep_with_state(action.rethink_pause, &state).await;

                            // Retype
                            for rc in action.text_to_retype.chars() {
                                let retype_delay = calculate_delay(rc, None, 0, &config, &mut rng);
                                type_char_helper(rc, retype_delay.as_millis() as u64, position, true, &injector_tx, &app_handle, &state).await;
                            }
                        }
                    }

                    // --- WORD SUBSTITUTION check (at word start) ---
                    // Look ahead to get the next full word, then maybe start typing a synonym instead
                    if is_word_boundary && config.word_substitution.enabled {
                        // Peek ahead to get the next word
                        let next_word: String = chars[i+1..].iter()
                            .take_while(|ch| !ch.is_whitespace() && !is_sentence_end(**ch))
                            .collect();

                        if next_word.len() >= 3 && word_sub_tracker.should_trigger(&config.word_substitution, &mut rng) {
                            if let Some(action) = word_sub_tracker.generate_sequence(&next_word, &config.word_substitution, &mut rng) {
                                word_sub_tracker.reset();

                                let synonym: String = action.partial_chars.iter().collect::<String>() + "…";
                                let _ = app_handle.emit("typing-synonym", TypingSynonymEvent {
                                    original: synonym,
                                    replacement: action.correct_word.clone(),
                                });

                                // First, type the current boundary char (space/newline)
                                let delay = calculate_delay(c, prev_char, 0, &config, &mut rng);
                                type_char_helper(c, delay.as_millis() as u64, position, true, &injector_tx, &app_handle, &state).await;
                                recent_chars.push(c);
                                position += 1;

                                // Type partial wrong word
                                for pc in &action.partial_chars {
                                    type_char_helper(*pc, rng.gen_range(80..=150), position, true, &injector_tx, &app_handle, &state).await;
                                }

                                // Notice the mistake
                                async_sleep_with_state(action.notice_pause, &state).await;

                                // Backspace the partial chars
                                for _ in 0..action.partial_chars.len() {
                                    backspace_helper(action.backspace_speed.as_millis() as u64, position, &injector_tx, &app_handle, &state).await;
                                }

                                // Pause before correct word
                                async_sleep_with_state(action.pre_correct_pause, &state).await;

                                // Type the correct word
                                for wc in action.correct_word.chars() {
                                    let wd = calculate_delay(wc, None, 0, &config, &mut rng);
                                    type_char_helper(wc, wd.as_millis() as u64, position, true, &injector_tx, &app_handle, &state).await;
                                }

                                // Skip past the word we just typed
                                i += 1 + next_word.len();
                                position += next_word.len();
                                current_word = next_word;
                                continue;
                            }
                        }
                    }

                    // --- Rule 11: Error injection check ---
                    let mut is_error = false;
                    if error_system.should_inject_error(
                        position,
                        c,
                        &config.error_config,
                        &config.error_clustering,
                        fatigue.error_rate_multiplier,
                        &mut rng,
                    ) {
                        is_error = true;
                        errors_made += 1;
                        let error_type = error_system.select_error_type(&config.error_config, &mut rng);
                        let error_action = error_system.generate_error(c, next_char, error_type, &mut rng);

                        // Type the error
                        match &error_action {
                            ErrorAction::Substitute { actual, .. } => {
                                let delay = calculate_delay(*actual, prev_char, word_pos, &config, &mut rng);
                                let _ = app_handle.emit("typing-error", TypingErrorEvent {
                                    position, intended: c, actual: actual.to_string(), error_type: "substitution".to_string(),
                                });
                                type_char_helper(*actual, delay.as_millis() as u64, position, false, &injector_tx, &app_handle, &state).await;
                            }
                            ErrorAction::Insert { extra, intended } => {
                                let delay = calculate_delay(*extra, prev_char, word_pos, &config, &mut rng);
                                let _ = app_handle.emit("typing-error", TypingErrorEvent {
                                    position, intended: c, actual: extra.to_string(), error_type: "insertion".to_string(),
                                });
                                type_char_helper(*extra, delay.as_millis() as u64, position, false, &injector_tx, &app_handle, &state).await;
                            }
                            ErrorAction::Double { intended } => {
                                let delay = calculate_delay(*intended, prev_char, word_pos, &config, &mut rng);
                                type_char_helper(*intended, delay.as_millis() as u64, position, true, &injector_tx, &app_handle, &state).await;
                                let delay2 = rng.gen_range(30..=60);
                                let _ = app_handle.emit("typing-error", TypingErrorEvent {
                                    position, intended: c, actual: format!("{}{}", intended, intended), error_type: "double".to_string(),
                                });
                                type_char_helper(*intended, delay2, position, false, &injector_tx, &app_handle, &state).await;
                            }
                            ErrorAction::Omit { .. } => {
                                let _ = app_handle.emit("typing-error", TypingErrorEvent {
                                    position, intended: c, actual: String::new(), error_type: "omission".to_string(),
                                });
                            }
                            ErrorAction::Transpose { first, second } => {
                                let delay1 = calculate_delay(*second, prev_char, word_pos, &config, &mut rng);
                                let _ = app_handle.emit("typing-error", TypingErrorEvent {
                                    position, intended: c, actual: format!("{}{}", second, first), error_type: "transposition".to_string(),
                                });
                                type_char_helper(*second, delay1.as_millis() as u64, position, false, &injector_tx, &app_handle, &state).await;
                                let delay2 = calculate_delay(*first, Some(*second), word_pos + 1, &config, &mut rng);
                                type_char_helper(*first, delay2.as_millis() as u64, position + 1, false, &injector_tx, &app_handle, &state).await;
                                if next_char.is_some() {
                                    i += 1;
                                    position += 1;
                                }
                            }
                            ErrorAction::WrongCaps { actual, .. } => {
                                let delay = calculate_delay(*actual, prev_char, word_pos, &config, &mut rng);
                                let _ = app_handle.emit("typing-error", TypingErrorEvent {
                                    position, intended: c, actual: actual.to_string(), error_type: "wrong_caps".to_string(),
                                });
                                type_char_helper(*actual, delay.as_millis() as u64, position, false, &injector_tx, &app_handle, &state).await;
                            }
                        }

                        // Rule 12: Error correction
                        let correction = error_system.generate_correction(&error_action, &config.correction_config, &mut rng);
                        if !correction.is_empty() {
                            corrections_made += 1;
                            let mut bs_count = 0;
                            let mut retyped = String::new();
                            for step in &correction {
                                match step {
                                    CorrectionStep::Wait(d) => {
                                        async_sleep_with_state(*d, &state).await;
                                    }
                                    CorrectionStep::Backspace => {
                                        bs_count += 1;
                                        backspace_helper(65, position, &injector_tx, &app_handle, &state).await;
                                    }
                                    CorrectionStep::Type(ch) => {
                                        retyped.push(*ch);
                                        type_char_helper(*ch, 60, position, true, &injector_tx, &app_handle, &state).await;
                                    }
                                }
                            }
                            let _ = app_handle.emit("typing-correction", TypingCorrectionEvent {
                                position, backspace_count: bs_count, retyped,
                            });
                        }
                    } else {
                        // Normal character — no error
                        let delay = calculate_delay_with_unfamiliar(
                            c, prev_char, word_pos, &config, in_unfamiliar_word, &mut rng,
                        );

                        // Apply fatigue speed multiplier
                        let adjusted_delay = Duration::from_millis(
                            (delay.as_millis() as f64 / fatigue.speed_multiplier) as u64
                        );

                        type_char_helper(c, adjusted_delay.as_millis() as u64, position, true, &injector_tx, &app_handle, &state).await;
                    }

                    recent_chars.push(c);
                    if recent_chars.len() > 50 {
                        recent_chars.drain(0..25);
                    }
                    if recent_text.len() > 500 {
                        recent_text = recent_text[250..].to_string();
                    }

                    // Rule 8: Punctuation pause (using configurable per-punctuation settings)
                    if config.punctuation_pauses_enabled {
                        if let Some(pause) = punctuation_pause_with_config(c, &config.punctuation_pause_config, &mut rng) {
                            let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                position,
                                pause_type: "punctuation".to_string(),
                                duration_ms: pause.as_millis() as u64,
                            });
                            async_sleep_with_state(pause, &state).await;
                        }
                    }

                    // Rule 9: Paragraph pause
                    if c == '\n' && next_char == Some('\n') {
                        let pause = paragraph_pause(&config.paragraph_pause, &mut rng);
                        if !pause.is_zero() {
                            let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                position,
                                pause_type: "paragraph".to_string(),
                                duration_ms: pause.as_millis() as u64,
                            });
                            async_sleep_with_state(pause, &state).await;
                        }
                    }

                    // Rule 10: Thinking pause at sentence/clause boundaries
                    if (is_sentence_end(c) || is_clause_boundary(c)) && config.thinking_pause.enabled {
                        if should_think(&config.thinking_pause, &mut rng) {
                            let pause = thinking_pause(&config.thinking_pause, &mut rng);
                            let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                position,
                                pause_type: "thinking".to_string(),
                                duration_ms: pause.as_millis() as u64,
                            });
                            async_sleep_with_state(pause, &state).await;
                        }
                    }

                    // Burst-pause check
                    if let Some(ref mut bt) = burst_tracker {
                        if let Some(bp_config) = &config.burst_pause {
                            bt.tick();
                            if bt.should_pause() {
                                let pause = bt.get_pause_and_reset(bp_config, &mut rng);
                                let _ = app_handle.emit("typing-pause", TypingPauseEvent {
                                    position,
                                    pause_type: "burst".to_string(),
                                    duration_ms: pause.as_millis() as u64,
                                });
                                async_sleep_with_state(pause, &state).await;
                            }
                        }
                    }

                    position += 1;

                    // Emit progress event
                    let elapsed = start_time.elapsed();
                    let elapsed_minutes = elapsed.as_secs_f64() / 60.0;
                    let current_wpm = if elapsed_minutes > 0.0 {
                        (position as f64 / 5.0) / elapsed_minutes
                    } else {
                        0.0
                    };

                    let _ = app_handle.emit("typing-progress", TypingProgressEvent {
                        position,
                        total: total_chars,
                        current_wpm,
                        target_wpm: config.wpm,
                        is_error,
                        is_pause: false,
                        elapsed_ms: elapsed.as_millis() as u64,
                        error_count: errors_made,
                        char_typed: Some(c),
                        fatigue_phase: match fatigue.phase {
                            FatiguePhase::None => 0,
                            FatiguePhase::Phase1 => 1,
                            FatiguePhase::Phase2 => 2,
                        },
                    });

                    i += 1;
                }
            }
        }
    }

    // Stop the injector thread
    if let Some(tx) = injector_tx {
        let _ = tx.send(InjectorCommand::Stop);
    }

    // Mark as completed
    {
        let mut s = state.lock().unwrap();
        *s = EngineState::Completed;
    }

    let stats = build_stats(total_chars, errors_made, corrections_made, start_time);

    let _ = app_handle.emit("typing-complete", TypingCompleteEvent {
        total_chars: stats.total_chars,
        total_errors: stats.total_errors,
        total_corrections: stats.total_corrections,
        elapsed_ms: stats.elapsed_ms,
        average_wpm: stats.average_wpm,
    });
    let _ = app_handle.emit("typing-state", TypingStateEvent { state: "completed".to_string() });

    stats
}

fn build_stats(total_chars: usize, errors: usize, corrections: usize, start: Instant) -> TypingStats {
    let elapsed = start.elapsed();
    let elapsed_minutes = elapsed.as_secs_f64() / 60.0;
    let avg_wpm = if elapsed_minutes > 0.0 {
        (total_chars as f64 / 5.0) / elapsed_minutes
    } else {
        0.0
    };
    TypingStats {
        total_chars,
        total_errors: errors,
        total_corrections: corrections,
        elapsed_ms: elapsed.as_millis() as u64,
        average_wpm: avg_wpm,
    }
}

/// Sleep that respects pause/cancel state
async fn async_sleep_with_state(duration: Duration, state: &SharedState) {
    let step = Duration::from_millis(50);
    let mut remaining = duration;

    while remaining > Duration::ZERO {
        let sleep_time = remaining.min(step);
        tokio::time::sleep(sleep_time).await;
        remaining = remaining.saturating_sub(sleep_time);

        let current = { *state.lock().unwrap() };
        match current {
            EngineState::Cancelled => return,
            EngineState::Paused => {
                loop {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    let s = { *state.lock().unwrap() };
                    if s != EngineState::Paused {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}
