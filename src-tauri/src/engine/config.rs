use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ParagraphPauseLevel {
    None,
    Brief,
    Short,
    Normal,
    Long,
    VeryLong,
    ExtendedBreak,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ThinkingPausePreset {
    Brief,
    Short,
    Normal,
    Medium,
    Long,
    VeryLong,
    ExtremelyLong,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CorrectionSpeed {
    Instant,
    Quick,
    Normal,
    Slow,
    VerySlow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingPauseConfig {
    pub enabled: bool,
    pub frequency: f64,         // 0.03 - 0.08 (3-8%)
    pub preset: ThinkingPausePreset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorConfig {
    pub enabled: bool,
    pub error_rate: f64,        // 0.01 - 0.10
    pub substitution_weight: f64,
    pub insertion_weight: f64,
    pub omission_weight: f64,
    pub double_letter_weight: f64,
    pub transposition_weight: f64,
    #[serde(default)]
    pub wrong_caps_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionConfig {
    pub enabled: bool,
    pub speed: CorrectionSpeed,
    pub over_backspace_chance: f64, // 0.10 - 0.15
    /// If Some, overrides the preset detection delay range
    #[serde(default)]
    pub custom_correction_delay: Option<(u64, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueConfig {
    pub enabled: bool,
    pub onset_minutes: f64,     // 15-20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroCorrectionConfig {
    pub enabled: bool,
    pub chance: f64,            // 0.02 - 0.04
    pub min_chars_between: usize, // 15-20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondThoughtsConfig {
    pub enabled: bool,
    pub chance: f64,            // 0.01 - 0.03
    pub min_words_between: usize, // 5-8
    /// Chance to replace a word with a synonym during second thoughts (0.0-1.0)
    #[serde(default = "default_synonym_chance")]
    pub synonym_chance: f64,
}

fn default_synonym_chance() -> f64 {
    0.30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstPauseConfig {
    pub enabled: bool,
    pub min_burst_length: usize,  // 5
    pub max_burst_length: usize,  // 20
    pub min_pause_ms: u64,        // 200
    pub max_pause_ms: u64,        // 800
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorClusterConfig {
    pub enabled: bool,
    pub multiplier: f64,     // 2.0 - 3.0
    pub range: usize,        // 5-10 chars
}

// =========================================================================
// New config types for features 2-6, 8-9
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSubstitutionConfig {
    pub enabled: bool,
    /// Chance per word boundary (0.01-0.05)
    pub chance: f64,
    /// How many chars of the wrong word to type before catching (3-5)
    pub partial_chars: usize,
    /// Minimum words between triggers
    pub min_words_between: usize,
}

impl Default for WordSubstitutionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            chance: 0.03,
            partial_chars: 4,
            min_words_between: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HesitationBackspaceConfig {
    pub enabled: bool,
    /// Chance per character (0.005-0.03)
    pub chance: f64,
    /// Minimum chars between triggers
    pub min_chars_between: usize,
}

impl Default for HesitationBackspaceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            chance: 0.01,
            min_chars_between: 25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidWordPauseConfig {
    pub enabled: bool,
    /// Chance per character while mid-word (0.005-0.02)
    pub chance: f64,
    /// Minimum chars between triggers
    pub min_chars_between: usize,
    /// Minimum pause duration in ms
    pub pause_ms_min: u64,
    /// Maximum pause duration in ms
    pub pause_ms_max: u64,
}

impl Default for MidWordPauseConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            chance: 0.01,
            min_chars_between: 30,
            pause_ms_min: 300,
            pause_ms_max: 1500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceRestartConfig {
    pub enabled: bool,
    /// Chance per sentence boundary (0.002-0.01)
    pub chance: f64,
    /// Minimum sentences between triggers
    pub min_sentences_between: usize,
}

impl Default for SentenceRestartConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            chance: 0.005,
            min_sentences_between: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunctuationPauseConfig {
    pub enabled: bool,
    /// (min_ms, max_ms) for period pause
    pub period_ms: (u64, u64),
    /// (min_ms, max_ms) for comma pause
    pub comma_ms: (u64, u64),
    /// (min_ms, max_ms) for question mark pause
    pub question_ms: (u64, u64),
    /// (min_ms, max_ms) for exclamation mark pause
    pub exclamation_ms: (u64, u64),
    /// (min_ms, max_ms) for colon pause
    pub colon_ms: (u64, u64),
    /// (min_ms, max_ms) for semicolon pause
    pub semicolon_ms: (u64, u64),
}

impl Default for PunctuationPauseConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            period_ms: (800, 1200),
            comma_ms: (400, 600),
            question_ms: (900, 1300),
            exclamation_ms: (900, 1300),
            colon_ms: (500, 700),
            semicolon_ms: (450, 650),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoPauseConfig {
    pub enabled: bool,
    /// Pause every N minutes
    pub interval_minutes: f64,
    /// Pause for M minutes
    pub duration_minutes: f64,
}

impl Default for AutoPauseConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_minutes: 30.0,
            duration_minutes: 5.0,
        }
    }
}

// =========================================================================
// Main EngineConfig
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    // Rule 1: Base speed
    pub wpm: f64,
    // Rule 2: Speed variation (0.08 - 0.30)
    pub speed_variation: f64,
    // Rule 3: Digraph timing
    pub digraph_enabled: bool,
    // Rules 4-6: Keyboard modifiers
    pub modifiers_enabled: bool,
    // Rule 7: Word boundary
    pub word_boundary_enabled: bool,
    // Rule 8: Punctuation pauses
    pub punctuation_pauses_enabled: bool,
    // Rule 9: Paragraph pauses
    pub paragraph_pause: ParagraphPauseLevel,
    // Rule 10: Thinking pauses
    pub thinking_pause: ThinkingPauseConfig,
    // Rule 11: Errors
    pub error_config: ErrorConfig,
    // Rule 12: Error correction
    pub correction_config: CorrectionConfig,
    // Rule 13: Rollover typing
    pub rollover_enabled: bool,
    pub rollover_chance: f64,
    // Rule 14: Fatigue
    pub fatigue_config: FatigueConfig,
    // Rule 15: Micro-corrections
    pub micro_correction: MicroCorrectionConfig,
    // Rule 16: Second thoughts
    pub second_thoughts: SecondThoughtsConfig,
    // Nice-to-have: Burst-pause
    pub burst_pause: Option<BurstPauseConfig>,
    // Nice-to-have: Error clustering
    pub error_clustering: ErrorClusterConfig,

    // --- New features ---

    /// Word substitution behavior: start typing synonym, catch mistake, correct
    #[serde(default)]
    pub word_substitution: WordSubstitutionConfig,
    /// Hesitation: random backspace of correct chars
    #[serde(default)]
    pub hesitation_backspace: HesitationBackspaceConfig,
    /// Hesitation: mid-word pause (freeze)
    #[serde(default)]
    pub mid_word_pause: MidWordPauseConfig,
    /// Hesitation: sentence restart
    #[serde(default)]
    pub sentence_restart: SentenceRestartConfig,
    /// Per-punctuation pause controls
    #[serde(default)]
    pub punctuation_pause_config: PunctuationPauseConfig,
    /// Unfamiliar word slowdown
    #[serde(default)]
    pub unfamiliar_word_slowdown: bool,
    #[serde(default = "default_unfamiliar_word_multiplier")]
    pub unfamiliar_word_multiplier: f64,
    /// Auto-pause configuration
    #[serde(default)]
    pub auto_pause: AutoPauseConfig,
}

fn default_unfamiliar_word_multiplier() -> f64 {
    1.5
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            // Average typist: 65 WPM
            wpm: 65.0,
            speed_variation: 0.18,
            digraph_enabled: true,
            modifiers_enabled: true,
            word_boundary_enabled: true,
            punctuation_pauses_enabled: true,
            paragraph_pause: ParagraphPauseLevel::Normal,
            thinking_pause: ThinkingPauseConfig {
                enabled: true,
                frequency: 0.05,
                preset: ThinkingPausePreset::Normal,
            },
            error_config: ErrorConfig {
                enabled: true,
                error_rate: 0.03,
                substitution_weight: 0.45,
                insertion_weight: 0.18,
                omission_weight: 0.17,
                double_letter_weight: 0.12,
                transposition_weight: 0.08,
                wrong_caps_weight: 0.0,
            },
            correction_config: CorrectionConfig {
                enabled: true,
                speed: CorrectionSpeed::Quick,
                over_backspace_chance: 0.12,
                custom_correction_delay: None,
            },
            rollover_enabled: false,
            rollover_chance: 0.0,
            fatigue_config: FatigueConfig {
                enabled: true,
                onset_minutes: 18.0,
            },
            micro_correction: MicroCorrectionConfig {
                enabled: true,
                chance: 0.03,
                min_chars_between: 18,
            },
            second_thoughts: SecondThoughtsConfig {
                enabled: true,
                chance: 0.02,
                min_words_between: 6,
                synonym_chance: 0.30,
            },
            burst_pause: None,
            error_clustering: ErrorClusterConfig {
                enabled: true,
                multiplier: 2.5,
                range: 7,
            },
            // New features — all disabled by default
            word_substitution: WordSubstitutionConfig::default(),
            hesitation_backspace: HesitationBackspaceConfig::default(),
            mid_word_pause: MidWordPauseConfig::default(),
            sentence_restart: SentenceRestartConfig::default(),
            punctuation_pause_config: PunctuationPauseConfig::default(),
            unfamiliar_word_slowdown: false,
            unfamiliar_word_multiplier: 1.5,
            auto_pause: AutoPauseConfig::default(),
        }
    }
}

impl ParagraphPauseLevel {
    pub fn range_ms(&self) -> (u64, u64) {
        match self {
            ParagraphPauseLevel::None => (0, 0),
            ParagraphPauseLevel::Brief => (1000, 3000),
            ParagraphPauseLevel::Short => (3000, 8000),
            ParagraphPauseLevel::Normal => (5000, 15000),
            ParagraphPauseLevel::Long => (15000, 45000),
            ParagraphPauseLevel::VeryLong => (45000, 120000),
            ParagraphPauseLevel::ExtendedBreak => (90000, 300000),
        }
    }
}

impl ThinkingPausePreset {
    pub fn range_ms(&self) -> (u64, u64) {
        match self {
            ThinkingPausePreset::Brief => (500, 2000),
            ThinkingPausePreset::Short => (1000, 4000),
            ThinkingPausePreset::Normal => (2000, 6000),
            ThinkingPausePreset::Medium => (4000, 10000),
            ThinkingPausePreset::Long => (8000, 20000),
            ThinkingPausePreset::VeryLong => (15000, 45000),
            ThinkingPausePreset::ExtremelyLong => (30000, 120000),
        }
    }
}

impl CorrectionSpeed {
    pub fn detection_delay_ms(&self) -> (u64, u64) {
        match self {
            CorrectionSpeed::Instant => (150, 250),
            CorrectionSpeed::Quick => (200, 400),
            CorrectionSpeed::Normal => (350, 550),
            CorrectionSpeed::Slow => (500, 800),
            CorrectionSpeed::VerySlow => (1000, 2000),
        }
    }
}
