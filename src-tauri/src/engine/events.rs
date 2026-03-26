use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TypingProgressEvent {
    pub position: usize,
    pub total: usize,
    pub current_wpm: f64,
    pub target_wpm: f64,
    pub is_error: bool,
    pub is_pause: bool,
    pub elapsed_ms: u64,
    pub error_count: usize,
    pub char_typed: Option<char>,
    pub fatigue_phase: u8, // 0=none, 1=phase1, 2=phase2
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingCharEvent {
    pub position: usize,
    pub char_typed: char,
    pub delay_ms: u64,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingErrorEvent {
    pub position: usize,
    pub intended: char,
    pub actual: String,
    pub error_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingCorrectionEvent {
    pub position: usize,
    pub backspace_count: usize,
    pub retyped: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingPauseEvent {
    pub position: usize,
    pub pause_type: String, // "thinking", "paragraph", "inline", "burst", "punctuation"
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingCompleteEvent {
    pub total_chars: usize,
    pub total_errors: usize,
    pub total_corrections: usize,
    pub elapsed_ms: u64,
    pub average_wpm: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingStateEvent {
    pub state: String, // "idle", "running", "paused", "completed", "cancelled", "countdown"
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingCountdownEvent {
    pub seconds_left: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypingSynonymEvent {
    pub original: String,
    pub replacement: String,
}
