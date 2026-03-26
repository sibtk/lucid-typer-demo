use std::time::Duration;
use rand::Rng;

use crate::engine::config::{
    EngineConfig, FatigueConfig, MicroCorrectionConfig, SecondThoughtsConfig, BurstPauseConfig,
    WordSubstitutionConfig, HesitationBackspaceConfig, MidWordPauseConfig, SentenceRestartConfig,
};
use crate::engine::digraph::is_different_hands;
use crate::engine::thesaurus;

/// Fatigue state tracker (Rule 14)
pub struct FatigueTracker {
    onset_minutes: f64,
}

impl FatigueTracker {
    pub fn new(config: &FatigueConfig) -> Self {
        Self {
            onset_minutes: config.onset_minutes,
        }
    }

    /// Get the current fatigue multipliers based on elapsed time
    pub fn get_multipliers(&self, elapsed_minutes: f64, enabled: bool) -> FatigueMultipliers {
        if !enabled || elapsed_minutes < self.onset_minutes {
            return FatigueMultipliers {
                error_rate_multiplier: 1.0,
                speed_multiplier: 1.0,
                phase: FatiguePhase::None,
            };
        }

        let time_since_onset = elapsed_minutes - self.onset_minutes;

        if time_since_onset < 30.0 {
            // Phase 1: errors increase, speed maintained
            let error_mult = 1.0 + (time_since_onset / 30.0 * 0.25);
            FatigueMultipliers {
                error_rate_multiplier: error_mult,
                speed_multiplier: 1.0,
                phase: FatiguePhase::Phase1,
            }
        } else {
            // Phase 2: both degrade
            let error_mult = 1.25 + ((time_since_onset - 30.0) / 60.0 * 0.15);
            let speed_mult = 1.0 - ((time_since_onset - 30.0) / 60.0 * 0.10);
            FatigueMultipliers {
                error_rate_multiplier: error_mult.min(2.0),
                speed_multiplier: speed_mult.max(0.75),
                phase: FatiguePhase::Phase2,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FatigueMultipliers {
    pub error_rate_multiplier: f64,
    pub speed_multiplier: f64,
    pub phase: FatiguePhase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FatiguePhase {
    None,
    Phase1,
    Phase2,
}

/// Check if rollover typing should be used (Rule 13)
pub fn should_rollover(
    prev_char: char,
    current_char: char,
    config: &EngineConfig,
    rng: &mut impl Rng,
) -> bool {
    if !config.rollover_enabled || config.wpm < 80.0 {
        return false;
    }
    if !is_different_hands(prev_char, current_char) {
        return false;
    }
    rng.gen::<f64>() < config.rollover_chance
}

/// Rollover overlap duration (how much the next key overlaps with the current)
pub fn rollover_overlap(rng: &mut impl Rng) -> Duration {
    Duration::from_millis(rng.gen_range(10..=30))
}

/// Micro-correction tracker (Rule 15)
pub struct MicroCorrectionTracker {
    chars_since_last: usize,
}

impl MicroCorrectionTracker {
    pub fn new() -> Self {
        Self { chars_since_last: 0 }
    }

    pub fn tick(&mut self) {
        self.chars_since_last += 1;
    }

    pub fn should_trigger(
        &self,
        config: &MicroCorrectionConfig,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }
        if self.chars_since_last < config.min_chars_between {
            return false;
        }
        rng.gen::<f64>() < config.chance
    }

    pub fn reset(&mut self) {
        self.chars_since_last = 0;
    }

    /// Generate the micro-correction sequence: backspace 1-3 chars and retype
    pub fn generate_sequence(&self, recent_chars: &[char], rng: &mut impl Rng) -> MicroCorrectionAction {
        let backspace_count = rng.gen_range(1..=3).min(recent_chars.len());
        let chars_to_retype: Vec<char> = recent_chars[recent_chars.len() - backspace_count..].to_vec();

        let notice_pause = Duration::from_millis(rng.gen_range(200..=500));
        let inter_backspace = Duration::from_millis(rng.gen_range(40..=80));
        let pre_retype_pause = Duration::from_millis(rng.gen_range(100..=250));

        MicroCorrectionAction {
            backspace_count,
            chars_to_retype,
            notice_pause,
            inter_backspace,
            pre_retype_pause,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MicroCorrectionAction {
    pub backspace_count: usize,
    pub chars_to_retype: Vec<char>,
    pub notice_pause: Duration,
    pub inter_backspace: Duration,
    pub pre_retype_pause: Duration,
}

/// Second thoughts tracker (Rule 16)
pub struct SecondThoughtsTracker {
    words_since_last: usize,
}

impl SecondThoughtsTracker {
    pub fn new() -> Self {
        Self { words_since_last: 0 }
    }

    pub fn on_word_end(&mut self) {
        self.words_since_last += 1;
    }

    pub fn should_trigger(
        &self,
        config: &SecondThoughtsConfig,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }
        if self.words_since_last < config.min_words_between {
            return false;
        }
        rng.gen::<f64>() < config.chance
    }

    pub fn reset(&mut self) {
        self.words_since_last = 0;
    }

    /// Generate second thoughts action: delete 2-5 words and retype (with optional synonym replacement)
    pub fn generate_sequence(
        &self,
        recent_text: &str,
        config: &SecondThoughtsConfig,
        rng: &mut impl Rng,
    ) -> SecondThoughtsAction {
        let words: Vec<&str> = recent_text.split_whitespace().collect();
        let words_to_delete = rng.gen_range(2..=5).min(words.len());

        // Calculate chars to delete (including spaces)
        let delete_from = words.len().saturating_sub(words_to_delete);
        let text_to_delete: String = words[delete_from..].join(" ");
        let chars_to_delete = text_to_delete.len() + if delete_from > 0 { 1 } else { 0 }; // +1 for leading space

        // Possibly replace words with synonyms
        let mut retyped_words: Vec<String> = Vec::new();
        let mut synonyms_used: Vec<(String, String)> = Vec::new();
        for word in &words[delete_from..] {
            if rng.gen::<f64>() < config.synonym_chance {
                if let Some(synonym) = thesaurus::get_synonym(word, rng) {
                    synonyms_used.push((word.to_string(), synonym.clone()));
                    retyped_words.push(synonym);
                    continue;
                }
            }
            retyped_words.push(word.to_string());
        }
        let text_to_retype = retyped_words.join(" ");

        let rethink_pause = Duration::from_millis(rng.gen_range(500..=1500));
        let reformulate_pause = Duration::from_millis(rng.gen_range(1000..=2500));
        let delete_speed = Duration::from_millis(rng.gen_range(30..=60));

        SecondThoughtsAction {
            chars_to_delete,
            text_to_retype,
            rethink_pause,
            reformulate_pause,
            delete_speed,
            synonyms_used,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecondThoughtsAction {
    pub chars_to_delete: usize,
    pub text_to_retype: String,
    pub rethink_pause: Duration,
    pub reformulate_pause: Duration,
    pub delete_speed: Duration,
    pub synonyms_used: Vec<(String, String)>,
}

/// Burst-pause pattern tracker
pub struct BurstPauseTracker {
    chars_in_current_burst: usize,
    current_burst_length: usize,
}

impl BurstPauseTracker {
    pub fn new(config: &BurstPauseConfig, rng: &mut impl Rng) -> Self {
        let burst_len = rng.gen_range(config.min_burst_length..=config.max_burst_length);
        Self {
            chars_in_current_burst: 0,
            current_burst_length: burst_len,
        }
    }

    pub fn tick(&mut self) {
        self.chars_in_current_burst += 1;
    }

    /// Check if we should pause between bursts
    pub fn should_pause(&self) -> bool {
        self.chars_in_current_burst >= self.current_burst_length
    }

    /// Get the pause duration and reset for next burst
    pub fn get_pause_and_reset(&mut self, config: &BurstPauseConfig, rng: &mut impl Rng) -> Duration {
        self.chars_in_current_burst = 0;
        self.current_burst_length = rng.gen_range(config.min_burst_length..=config.max_burst_length);
        Duration::from_millis(rng.gen_range(config.min_pause_ms..=config.max_pause_ms))
    }
}

// =========================================================================
// Feature 2: Word Substitution Behavior
// =========================================================================

/// Tracks word substitution events
pub struct WordSubstitutionTracker {
    words_since_last: usize,
}

impl WordSubstitutionTracker {
    pub fn new() -> Self {
        Self { words_since_last: 0 }
    }

    pub fn on_word_boundary(&mut self) {
        self.words_since_last += 1;
    }

    pub fn should_trigger(
        &self,
        config: &WordSubstitutionConfig,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }
        if self.words_since_last < config.min_words_between {
            return false;
        }
        rng.gen::<f64>() < config.chance
    }

    pub fn reset(&mut self) {
        self.words_since_last = 0;
    }

    /// Generate a word substitution action.
    /// Takes the current word about to be typed.
    /// Returns Some(action) if a synonym is found, None otherwise.
    pub fn generate_sequence(
        &self,
        current_word: &str,
        config: &WordSubstitutionConfig,
        rng: &mut impl Rng,
    ) -> Option<WordSubstitutionAction> {
        // Get a synonym of the current word (the "wrong" start)
        let synonym = thesaurus::get_synonym(current_word, rng)?;

        // How many chars of the synonym to type before catching the mistake
        let partial = config.partial_chars.min(synonym.len()).max(1);
        let partial_chars: Vec<char> = synonym.chars().take(partial).collect();

        let notice_pause = Duration::from_millis(rng.gen_range(300..=600));
        let backspace_speed = Duration::from_millis(rng.gen_range(40..=80));
        let pre_correct_pause = Duration::from_millis(rng.gen_range(200..=400));

        Some(WordSubstitutionAction {
            partial_chars,
            correct_word: current_word.to_string(),
            notice_pause,
            backspace_speed,
            pre_correct_pause,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WordSubstitutionAction {
    /// The partial chars of the wrong synonym to type
    pub partial_chars: Vec<char>,
    /// The correct word to type after correction
    pub correct_word: String,
    /// Pause when noticing the mistake
    pub notice_pause: Duration,
    /// Speed of backspacing the partial chars
    pub backspace_speed: Duration,
    /// Pause before typing the correct word
    pub pre_correct_pause: Duration,
}

// =========================================================================
// Feature 3a: Hesitation Backspace
// =========================================================================

/// Tracks hesitation backspace events (random delete of correct chars)
pub struct HesitationBackspaceTracker {
    chars_since_last: usize,
}

impl HesitationBackspaceTracker {
    pub fn new() -> Self {
        Self { chars_since_last: 0 }
    }

    pub fn tick(&mut self) {
        self.chars_since_last += 1;
    }

    pub fn should_trigger(
        &self,
        config: &HesitationBackspaceConfig,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }
        if self.chars_since_last < config.min_chars_between {
            return false;
        }
        rng.gen::<f64>() < config.chance
    }

    pub fn reset(&mut self) {
        self.chars_since_last = 0;
    }

    /// Generate hesitation backspace sequence: delete 1-3 correct chars, pause, retype them
    pub fn generate_sequence(&self, recent_chars: &[char], rng: &mut impl Rng) -> HesitationBackspaceAction {
        let backspace_count = rng.gen_range(1..=3).min(recent_chars.len());
        let chars_to_retype: Vec<char> = recent_chars[recent_chars.len() - backspace_count..].to_vec();

        let hesitation_pause = Duration::from_millis(rng.gen_range(400..=800));
        let backspace_speed = Duration::from_millis(rng.gen_range(40..=80));
        let pre_retype_pause = Duration::from_millis(rng.gen_range(100..=250));

        HesitationBackspaceAction {
            backspace_count,
            chars_to_retype,
            hesitation_pause,
            backspace_speed,
            pre_retype_pause,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HesitationBackspaceAction {
    pub backspace_count: usize,
    pub chars_to_retype: Vec<char>,
    /// Pause after deleting (uncertainty/nervousness pause)
    pub hesitation_pause: Duration,
    /// Speed of backspace keys
    pub backspace_speed: Duration,
    /// Pause before retyping
    pub pre_retype_pause: Duration,
}

// =========================================================================
// Feature 3b: Mid-Word Pause
// =========================================================================

/// Tracks mid-word pause events (freeze in the middle of typing)
pub struct MidWordPauseTracker {
    chars_since_last: usize,
}

impl MidWordPauseTracker {
    pub fn new() -> Self {
        Self { chars_since_last: 0 }
    }

    pub fn tick(&mut self) {
        self.chars_since_last += 1;
    }

    pub fn should_trigger(
        &self,
        config: &MidWordPauseConfig,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }
        if self.chars_since_last < config.min_chars_between {
            return false;
        }
        rng.gen::<f64>() < config.chance
    }

    pub fn reset(&mut self) {
        self.chars_since_last = 0;
    }

    /// Generate a mid-word pause duration
    pub fn generate_pause(&self, config: &MidWordPauseConfig, rng: &mut impl Rng) -> Duration {
        Duration::from_millis(rng.gen_range(config.pause_ms_min..=config.pause_ms_max))
    }
}

// =========================================================================
// Feature 3c: Sentence Restart
// =========================================================================

/// Tracks sentence restart events (delete entire last sentence and retype)
pub struct SentenceRestartTracker {
    sentences_since_last: usize,
}

impl SentenceRestartTracker {
    pub fn new() -> Self {
        Self { sentences_since_last: 0 }
    }

    pub fn on_sentence_end(&mut self) {
        self.sentences_since_last += 1;
    }

    pub fn should_trigger(
        &self,
        config: &SentenceRestartConfig,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }
        if self.sentences_since_last < config.min_sentences_between {
            return false;
        }
        rng.gen::<f64>() < config.chance
    }

    pub fn reset(&mut self) {
        self.sentences_since_last = 0;
    }

    /// Generate a sentence restart action.
    /// `last_sentence` is the text of the sentence just completed (up to the punctuation).
    pub fn generate_sequence(
        &self,
        last_sentence: &str,
        rng: &mut impl Rng,
    ) -> SentenceRestartAction {
        let chars_to_delete = last_sentence.len();
        let rethink_pause = Duration::from_millis(rng.gen_range(1000..=3000));
        let delete_speed = Duration::from_millis(rng.gen_range(25..=50));

        SentenceRestartAction {
            chars_to_delete,
            text_to_retype: last_sentence.to_string(),
            rethink_pause,
            delete_speed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SentenceRestartAction {
    pub chars_to_delete: usize,
    pub text_to_retype: String,
    /// Long pause before retyping (reconsidering)
    pub rethink_pause: Duration,
    /// Speed of backspace deletion
    pub delete_speed: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_fatigue_no_onset() {
        let config = FatigueConfig { enabled: true, onset_minutes: 18.0 };
        let tracker = FatigueTracker::new(&config);
        let m = tracker.get_multipliers(10.0, true);
        assert_eq!(m.phase, FatiguePhase::None);
        assert!((m.error_rate_multiplier - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fatigue_phase1() {
        let config = FatigueConfig { enabled: true, onset_minutes: 15.0 };
        let tracker = FatigueTracker::new(&config);
        let m = tracker.get_multipliers(30.0, true); // 15 min after onset
        assert_eq!(m.phase, FatiguePhase::Phase1);
        assert!(m.error_rate_multiplier > 1.0);
        assert!((m.speed_multiplier - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fatigue_phase2() {
        let config = FatigueConfig { enabled: true, onset_minutes: 15.0 };
        let tracker = FatigueTracker::new(&config);
        let m = tracker.get_multipliers(60.0, true); // 45 min after onset
        assert_eq!(m.phase, FatiguePhase::Phase2);
        assert!(m.error_rate_multiplier > 1.25);
        assert!(m.speed_multiplier < 1.0);
    }

    #[test]
    fn test_burst_pause_tracker() {
        let config = BurstPauseConfig {
            enabled: true,
            min_burst_length: 5,
            max_burst_length: 10,
            min_pause_ms: 200,
            max_pause_ms: 800,
        };
        let mut rng = thread_rng();
        let mut tracker = BurstPauseTracker::new(&config, &mut rng);

        // Tick through a burst
        for _ in 0..20 {
            tracker.tick();
            if tracker.should_pause() {
                let pause = tracker.get_pause_and_reset(&config, &mut rng);
                assert!(pause.as_millis() >= 200 && pause.as_millis() <= 800);
                break;
            }
        }
    }

    #[test]
    fn test_word_substitution_no_synonym() {
        let mut rng = thread_rng();
        let tracker = WordSubstitutionTracker::new();
        let config = WordSubstitutionConfig {
            enabled: true,
            chance: 1.0,
            partial_chars: 4,
            min_words_between: 0,
        };
        // A word with no synonym should return None
        let result = tracker.generate_sequence("xyzzy", &config, &mut rng);
        assert!(result.is_none());
    }

    #[test]
    fn test_word_substitution_with_synonym() {
        let mut rng = thread_rng();
        let tracker = WordSubstitutionTracker::new();
        let config = WordSubstitutionConfig {
            enabled: true,
            chance: 1.0,
            partial_chars: 3,
            min_words_between: 0,
        };
        let result = tracker.generate_sequence("good", &config, &mut rng);
        assert!(result.is_some());
        let action = result.unwrap();
        assert!(action.partial_chars.len() <= 3);
        assert_eq!(action.correct_word, "good");
    }

    #[test]
    fn test_hesitation_backspace() {
        let mut rng = thread_rng();
        let tracker = HesitationBackspaceTracker::new();
        let recent = vec!['h', 'e', 'l', 'l', 'o'];
        let action = tracker.generate_sequence(&recent, &mut rng);
        assert!(action.backspace_count >= 1 && action.backspace_count <= 3);
        assert_eq!(action.chars_to_retype.len(), action.backspace_count);
    }

    #[test]
    fn test_mid_word_pause() {
        let config = MidWordPauseConfig {
            enabled: true,
            chance: 1.0,
            min_chars_between: 0,
            pause_ms_min: 300,
            pause_ms_max: 1500,
        };
        let mut rng = thread_rng();
        let mut tracker = MidWordPauseTracker::new();
        tracker.tick();
        assert!(tracker.should_trigger(&config, &mut rng));
        let pause = tracker.generate_pause(&config, &mut rng);
        assert!(pause.as_millis() >= 300 && pause.as_millis() <= 1500);
    }

    #[test]
    fn test_sentence_restart() {
        let mut rng = thread_rng();
        let tracker = SentenceRestartTracker::new();
        let action = tracker.generate_sequence("Hello world.", &mut rng);
        assert_eq!(action.chars_to_delete, 12);
        assert_eq!(action.text_to_retype, "Hello world.");
        assert!(action.rethink_pause.as_millis() >= 1000 && action.rethink_pause.as_millis() <= 3000);
    }

    #[test]
    fn test_second_thoughts_with_synonym() {
        let mut rng = thread_rng();
        let tracker = SecondThoughtsTracker::new();
        let config = SecondThoughtsConfig {
            enabled: true,
            chance: 1.0,
            min_words_between: 0,
            synonym_chance: 1.0, // always try synonym
        };
        // Use text with a word that has synonyms
        let action = tracker.generate_sequence("the good thing is done", &config, &mut rng);
        // Should complete without panic; synonyms may or may not be found for all words
        assert!(action.chars_to_delete > 0);
        assert!(!action.text_to_retype.is_empty());
    }
}
