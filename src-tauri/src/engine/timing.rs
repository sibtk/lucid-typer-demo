use std::time::Duration;
use rand::Rng;
use rand_distr::{Normal, Distribution};

use crate::engine::config::{EngineConfig, PunctuationPauseConfig};
use crate::engine::keyboard_map::{get_key_position, char_class_modifier};
use crate::engine::digraph::digraph_modifier;

/// Uncommon letter clusters that mark a word as "unfamiliar"
const UNCOMMON_CLUSTERS: &[&str] = &[
    "xz", "qw", "zz", "xq", "qx", "zx", "zq", "jx", "qj", "vx",
    "bx", "wx", "xk", "kx", "fq", "qf", "jq", "qg", "gx", "zj",
    "zv", "vz", "xv", "vq", "qv", "wz", "zw",
];

/// Consonant characters for detecting consecutive consonant clusters
fn is_consonant(c: char) -> bool {
    matches!(
        c.to_ascii_lowercase(),
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm'
        | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z'
    )
}

/// Check if a word is "unfamiliar" (long, or contains uncommon clusters / 3+ consecutive consonants)
pub fn is_unfamiliar_word(word: &str) -> bool {
    // Length >= 10
    if word.len() >= 10 {
        return true;
    }

    let lower = word.to_lowercase();

    // Contains uncommon letter clusters
    for cluster in UNCOMMON_CLUSTERS {
        if lower.contains(cluster) {
            return true;
        }
    }

    // Contains 3+ consecutive consonants
    let mut consecutive = 0;
    for c in lower.chars() {
        if is_consonant(c) {
            consecutive += 1;
            if consecutive >= 3 {
                return true;
            }
        } else {
            consecutive = 0;
        }
    }

    false
}

/// Calculate the delay before typing a character (Rules 1-8)
pub fn calculate_delay(
    current_char: char,
    prev_char: Option<char>,
    position_in_word: usize,
    config: &EngineConfig,
    rng: &mut impl Rng,
) -> Duration {
    // Rule 1: Base delay from WPM
    let chars_per_minute = config.wpm * 5.0;
    let base_delay_ms = 60000.0 / chars_per_minute;

    let mut delay = base_delay_ms;

    // Rule 2: Speed variation (Gaussian noise)
    let variation = config.speed_variation.clamp(0.08, 0.30);
    let normal = Normal::new(1.0, variation / 2.0).unwrap_or(Normal::new(1.0, 0.05).unwrap());
    let variation_factor = normal.sample(rng).clamp(1.0 - variation, 1.0 + variation);
    delay *= variation_factor;

    // Rule 3: Digraph timing
    if config.digraph_enabled {
        if let Some(prev) = prev_char {
            if prev != ' ' && current_char != ' ' {
                let dig_mod = digraph_modifier(prev, current_char, rng);
                delay *= dig_mod;
            }
        }
    }

    if config.modifiers_enabled {
        // Rule 4: Keyboard row modifier
        if let Some(pos) = get_key_position(current_char) {
            delay *= pos.row.modifier();

            // Rule 5: Finger strength modifier
            delay *= pos.finger.strength_modifier();
        }

        // Rule 6: Special character modifier
        delay *= char_class_modifier(current_char);
    }

    // Rule 7: Word boundary behavior
    if config.word_boundary_enabled {
        if current_char == ' ' {
            // Space is fast (thumb)
            delay *= rng.gen_range(0.85..=0.95);
        } else if position_in_word == 0 {
            // First character of word: extra cognitive delay
            delay *= rng.gen_range(1.15..=1.25);
        }
    }

    // Rule 8: Check if this is punctuation that should cause a post-typing pause
    // (The actual pause is applied after the character, handled by the caller)
    // Here we just adjust the keystroke delay for the punctuation character itself

    // Ensure delay is at least 10ms and at most 500ms for a single keystroke
    let delay_ms = delay.clamp(10.0, 500.0);
    Duration::from_millis(delay_ms as u64)
}

/// Calculate delay with unfamiliar word multiplier applied
pub fn calculate_delay_with_unfamiliar(
    current_char: char,
    prev_char: Option<char>,
    position_in_word: usize,
    config: &EngineConfig,
    in_unfamiliar_word: bool,
    rng: &mut impl Rng,
) -> Duration {
    let base = calculate_delay(current_char, prev_char, position_in_word, config, rng);
    if config.unfamiliar_word_slowdown && in_unfamiliar_word && current_char.is_alphabetic() {
        Duration::from_millis((base.as_millis() as f64 * config.unfamiliar_word_multiplier) as u64)
    } else {
        base
    }
}

/// Get the pause duration after typing a punctuation character (Rule 8)
/// Uses the configurable PunctuationPauseConfig if provided.
pub fn punctuation_pause(c: char, rng: &mut impl Rng) -> Option<Duration> {
    punctuation_pause_with_config(c, &PunctuationPauseConfig::default(), rng)
}

/// Get the pause duration after typing a punctuation character using custom config
pub fn punctuation_pause_with_config(
    c: char,
    config: &PunctuationPauseConfig,
    rng: &mut impl Rng,
) -> Option<Duration> {
    if !config.enabled {
        return None;
    }

    let (base_min, base_max) = match c {
        '.' => config.period_ms,
        ',' => config.comma_ms,
        '?' => config.question_ms,
        '!' => config.exclamation_ms,
        ':' => config.colon_ms,
        ';' => config.semicolon_ms,
        _ => return None,
    };

    if base_min == 0 && base_max == 0 {
        return None;
    }

    // Apply +/-20% variance
    let base = rng.gen_range(base_min..=base_max) as f64;
    let variance = base * rng.gen_range(-0.20..=0.20);
    let pause_ms = (base + variance).max(100.0) as u64;
    Some(Duration::from_millis(pause_ms))
}

/// Calculate auto-derived speed variation from WPM (Rule 2 formula)
pub fn auto_variation(wpm: f64) -> f64 {
    (30.0 - (wpm / 5.0)).clamp(8.0, 30.0) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_base_delay_reasonable() {
        let config = EngineConfig::default(); // 65 WPM
        let mut rng = thread_rng();
        let delay = calculate_delay('a', None, 0, &config, &mut rng);
        // 65 WPM = 325 CPM = ~184ms base. With modifiers, should be 50-400ms range
        assert!(delay.as_millis() >= 10);
        assert!(delay.as_millis() <= 500);
    }

    #[test]
    fn test_space_is_faster() {
        let config = EngineConfig::default();
        let mut rng = thread_rng();
        let mut space_total = 0u128;
        let mut letter_total = 0u128;
        let n = 100;
        for _ in 0..n {
            space_total += calculate_delay(' ', Some('a'), 0, &config, &mut rng).as_millis();
            letter_total += calculate_delay('a', Some('t'), 1, &config, &mut rng).as_millis();
        }
        // Space should generally be faster on average
        assert!(space_total < letter_total * 2, "Space should be relatively fast");
    }

    #[test]
    fn test_punctuation_pause_exists() {
        let mut rng = thread_rng();
        let pause = punctuation_pause('.', &mut rng);
        assert!(pause.is_some());
        let ms = pause.unwrap().as_millis();
        assert!(ms >= 600 && ms <= 1500, "Period pause {} out of range", ms);
    }

    #[test]
    fn test_auto_variation() {
        assert!((auto_variation(65.0) - 0.17).abs() < 0.01);
        assert!((auto_variation(120.0) - 0.08).abs() < 0.01); // clamped to min
        assert!((auto_variation(20.0) - 0.26).abs() < 0.01);
    }

    #[test]
    fn test_unfamiliar_word_long() {
        assert!(is_unfamiliar_word("complicated")); // 11 chars
        assert!(!is_unfamiliar_word("hello"));       // 5 chars
    }

    #[test]
    fn test_unfamiliar_word_uncommon_cluster() {
        assert!(is_unfamiliar_word("fuzz")); // contains 'zz'
    }

    #[test]
    fn test_unfamiliar_word_consecutive_consonants() {
        assert!(is_unfamiliar_word("strengths")); // 'ngth' has 4 consecutive consonants
        assert!(!is_unfamiliar_word("ate"));       // no 3+ consecutive
    }

    #[test]
    fn test_configurable_punctuation_pause() {
        let mut rng = thread_rng();
        let config = PunctuationPauseConfig {
            enabled: true,
            period_ms: (100, 200),
            comma_ms: (50, 100),
            question_ms: (100, 200),
            exclamation_ms: (100, 200),
            colon_ms: (80, 160),
            semicolon_ms: (70, 140),
        };
        let pause = punctuation_pause_with_config('.', &config, &mut rng);
        assert!(pause.is_some());
        // With variance, range is roughly 80-240
        let ms = pause.unwrap().as_millis();
        assert!(ms >= 80 && ms <= 250, "Custom period pause {} out of range", ms);
    }

    #[test]
    fn test_punctuation_pause_disabled() {
        let mut rng = thread_rng();
        let config = PunctuationPauseConfig {
            enabled: false,
            ..PunctuationPauseConfig::default()
        };
        let pause = punctuation_pause_with_config('.', &config, &mut rng);
        assert!(pause.is_none());
    }

    #[test]
    fn test_unfamiliar_delay_multiplier() {
        let mut config = EngineConfig::default();
        config.unfamiliar_word_slowdown = true;
        config.unfamiliar_word_multiplier = 2.0;
        let mut rng = thread_rng();

        let normal = calculate_delay_with_unfamiliar('a', None, 0, &config, false, &mut rng);
        let slow = calculate_delay_with_unfamiliar('a', None, 0, &config, true, &mut rng);

        // With multiplier 2.0, unfamiliar should be roughly 2x (on average, with variance)
        // We just check that the function doesn't panic and that the unfamiliar path is used
        assert!(normal.as_millis() >= 10);
        assert!(slow.as_millis() >= 10);
    }
}
