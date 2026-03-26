use rand::Rng;
use std::time::Duration;

use crate::engine::config::{ErrorConfig, CorrectionConfig, ErrorClusterConfig};
use crate::engine::keyboard_map::{get_adjacent_keys, typo_danger_weight};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorType {
    Substitution,   // Wrong adjacent key (45%)
    Insertion,      // Extra key pressed (18%)
    Omission,       // Missed key (17%)
    DoubleLetter,   // Key pressed twice (12%)
    Transposition,  // Two chars swapped (8%)
    WrongCaps,      // Wrong case: 'a' instead of 'A' or vice versa
}

#[derive(Debug, Clone)]
pub enum ErrorAction {
    /// Replace intended char with wrong char
    Substitute { intended: char, actual: char },
    /// Insert an extra char before the intended
    Insert { extra: char, intended: char },
    /// Skip the intended char entirely
    Omit { intended: char },
    /// Type the intended char twice
    Double { intended: char },
    /// Swap this char with the next
    Transpose { first: char, second: char },
    /// Type with wrong case
    WrongCaps { intended: char, actual: char },
}

#[derive(Debug, Clone)]
pub enum CorrectionStep {
    /// Wait for a duration (detection delay, inter-key pause, etc.)
    Wait(Duration),
    /// Press backspace
    Backspace,
    /// Type a character
    Type(char),
}

pub struct ErrorSystem {
    last_error_pos: Option<usize>,
    chars_since_last_error: usize,
}

impl ErrorSystem {
    pub fn new() -> Self {
        Self {
            last_error_pos: None,
            chars_since_last_error: 0,
        }
    }

    /// Determine if an error should be injected at this position (Rule 11)
    pub fn should_inject_error(
        &mut self,
        char_index: usize,
        intended_char: char,
        config: &ErrorConfig,
        clustering: &ErrorClusterConfig,
        fatigue_multiplier: f64,
        rng: &mut impl Rng,
    ) -> bool {
        if !config.enabled {
            return false;
        }

        // Don't inject errors on whitespace or newlines
        if intended_char.is_whitespace() {
            self.chars_since_last_error += 1;
            return false;
        }

        let mut effective_rate = config.error_rate;

        // Apply fatigue multiplier
        effective_rate *= fatigue_multiplier;

        // Apply dangerous letter weighting
        effective_rate *= typo_danger_weight(intended_char);

        // Apply error clustering
        if clustering.enabled {
            if let Some(last_pos) = self.last_error_pos {
                let distance = char_index.saturating_sub(last_pos);
                if distance <= clustering.range {
                    effective_rate *= clustering.multiplier;
                }
            }
        }

        let should_error = rng.gen::<f64>() < effective_rate;
        if should_error {
            self.last_error_pos = Some(char_index);
            self.chars_since_last_error = 0;
        } else {
            self.chars_since_last_error += 1;
        }

        should_error
    }

    /// Select an error type based on configured weights
    pub fn select_error_type(&self, config: &ErrorConfig, rng: &mut impl Rng) -> ErrorType {
        let total = config.substitution_weight
            + config.insertion_weight
            + config.omission_weight
            + config.double_letter_weight
            + config.transposition_weight
            + config.wrong_caps_weight;

        let roll = rng.gen::<f64>() * total;
        let mut cumulative = 0.0;

        cumulative += config.substitution_weight;
        if roll < cumulative {
            return ErrorType::Substitution;
        }

        cumulative += config.insertion_weight;
        if roll < cumulative {
            return ErrorType::Insertion;
        }

        cumulative += config.omission_weight;
        if roll < cumulative {
            return ErrorType::Omission;
        }

        cumulative += config.double_letter_weight;
        if roll < cumulative {
            return ErrorType::DoubleLetter;
        }

        cumulative += config.transposition_weight;
        if roll < cumulative {
            return ErrorType::Transposition;
        }

        ErrorType::WrongCaps
    }

    /// Generate the actual error action
    pub fn generate_error(
        &self,
        intended: char,
        next_char: Option<char>,
        error_type: ErrorType,
        rng: &mut impl Rng,
    ) -> ErrorAction {
        match error_type {
            ErrorType::Substitution => {
                let adjacent = get_adjacent_keys(intended);
                let actual = if adjacent.is_empty() {
                    // Fallback: shift the char
                    ((intended as u8).wrapping_add(1)) as char
                } else {
                    adjacent[rng.gen_range(0..adjacent.len())]
                };
                // Preserve case
                let actual = if intended.is_uppercase() {
                    actual.to_ascii_uppercase()
                } else {
                    actual
                };
                ErrorAction::Substitute { intended, actual }
            }
            ErrorType::Insertion => {
                let adjacent = get_adjacent_keys(intended);
                let extra = if adjacent.is_empty() {
                    intended
                } else {
                    adjacent[rng.gen_range(0..adjacent.len())]
                };
                ErrorAction::Insert { extra, intended }
            }
            ErrorType::Omission => {
                ErrorAction::Omit { intended }
            }
            ErrorType::DoubleLetter => {
                ErrorAction::Double { intended }
            }
            ErrorType::Transposition => {
                let second = next_char.unwrap_or(intended);
                ErrorAction::Transpose { first: intended, second }
            }
            ErrorType::WrongCaps => {
                // Flip the case of the character
                let actual = if intended.is_uppercase() {
                    intended.to_ascii_lowercase()
                } else if intended.is_lowercase() {
                    intended.to_ascii_uppercase()
                } else {
                    // Non-alphabetic: fall back to substitution-like behavior
                    intended
                };
                ErrorAction::WrongCaps { intended, actual }
            }
        }
    }

    /// Generate the correction sequence for an error (Rule 12)
    pub fn generate_correction(
        &self,
        error: &ErrorAction,
        config: &CorrectionConfig,
        rng: &mut impl Rng,
    ) -> Vec<CorrectionStep> {
        if !config.enabled {
            return vec![];
        }

        let mut steps = Vec::new();

        // Use custom correction delay if provided, otherwise use preset
        let (det_min, det_max) = if let Some((custom_min, custom_max)) = config.custom_correction_delay {
            (custom_min, custom_max)
        } else {
            config.speed.detection_delay_ms()
        };

        // 1. Detection delay
        let detection_ms = rng.gen_range(det_min..=det_max);
        steps.push(CorrectionStep::Wait(Duration::from_millis(detection_ms)));

        // Determine how many chars to backspace
        let (backspace_count, chars_to_retype) = match error {
            ErrorAction::Substitute { intended, .. } => {
                if rng.gen::<f64>() < config.over_backspace_chance {
                    // Over-backspace: delete 1 extra correct char
                    let extra = rng.gen_range(1..=2);
                    (1 + extra, vec![*intended])
                } else {
                    (1, vec![*intended])
                }
            }
            ErrorAction::Insert { intended, extra: _ } => {
                // Need to delete the extra char
                (1, vec![*intended])
            }
            ErrorAction::Double { intended: _ } => {
                // Delete the doubled char
                (1, vec![])
            }
            ErrorAction::Transpose { first, second } => {
                // Delete both swapped chars and retype correctly
                (2, vec![*first, *second])
            }
            ErrorAction::Omit { intended: _ } => {
                // Omission: might notice and backspace to insert
                // Or might just type the missing char (handled differently)
                return vec![];
            }
            ErrorAction::WrongCaps { intended, .. } => {
                // Delete the wrong-cased char and retype with correct case
                (1, vec![*intended])
            }
        };

        // 2. Backspace keystrokes
        for _ in 0..backspace_count {
            let bs_delay = rng.gen_range(50..=80);
            steps.push(CorrectionStep::Backspace);
            steps.push(CorrectionStep::Wait(Duration::from_millis(bs_delay)));
        }

        // 3. Brief pause before retyping
        let pause_ms = rng.gen_range(30..=60);
        steps.push(CorrectionStep::Wait(Duration::from_millis(pause_ms)));

        // 4. Retype correct characters
        for c in chars_to_retype {
            steps.push(CorrectionStep::Type(c));
            let retype_delay = rng.gen_range(40..=80);
            steps.push(CorrectionStep::Wait(Duration::from_millis(retype_delay)));
        }

        // If over-backspaced, need to retype the extra chars that were correct
        if backspace_count > 1 {
            match error {
                ErrorAction::Substitute { .. } | ErrorAction::WrongCaps { .. } => {
                    // The extra backspaced chars need retyping — but we don't know them here.
                    // The caller handles this by tracking the buffer.
                    let realization_pause = rng.gen_range(100..=200);
                    steps.push(CorrectionStep::Wait(Duration::from_millis(realization_pause)));
                }
                _ => {}
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use crate::engine::config::{CorrectionSpeed, ErrorClusterConfig};

    #[test]
    fn test_error_type_weights() {
        let config = ErrorConfig {
            enabled: true,
            error_rate: 1.0, // 100% for testing
            substitution_weight: 0.45,
            insertion_weight: 0.18,
            omission_weight: 0.17,
            double_letter_weight: 0.12,
            transposition_weight: 0.08,
            wrong_caps_weight: 0.0,
        };

        let mut rng = thread_rng();
        let mut counts = [0u32; 5];
        let n = 10000;

        for _ in 0..n {
            let error_system = ErrorSystem::new();
            match error_system.select_error_type(&config, &mut rng) {
                ErrorType::Substitution => counts[0] += 1,
                ErrorType::Insertion => counts[1] += 1,
                ErrorType::Omission => counts[2] += 1,
                ErrorType::DoubleLetter => counts[3] += 1,
                ErrorType::Transposition => counts[4] += 1,
                ErrorType::WrongCaps => {} // weight is 0, shouldn't appear
            }
        }

        // Substitution should be the most common (~45%)
        assert!(counts[0] > counts[1]);
        assert!(counts[0] > counts[2]);
        // Transposition should be the least common (~8%)
        assert!(counts[4] < counts[1]);
    }

    #[test]
    fn test_wrong_caps_error() {
        let mut rng = thread_rng();
        let system = ErrorSystem::new();
        let action = system.generate_error('a', None, ErrorType::WrongCaps, &mut rng);
        match action {
            ErrorAction::WrongCaps { intended, actual } => {
                assert_eq!(intended, 'a');
                assert_eq!(actual, 'A');
            }
            _ => panic!("Expected WrongCaps"),
        }

        let action2 = system.generate_error('B', None, ErrorType::WrongCaps, &mut rng);
        match action2 {
            ErrorAction::WrongCaps { intended, actual } => {
                assert_eq!(intended, 'B');
                assert_eq!(actual, 'b');
            }
            _ => panic!("Expected WrongCaps"),
        }
    }

    #[test]
    fn test_wrong_caps_weight() {
        let config = ErrorConfig {
            enabled: true,
            error_rate: 1.0,
            substitution_weight: 0.0,
            insertion_weight: 0.0,
            omission_weight: 0.0,
            double_letter_weight: 0.0,
            transposition_weight: 0.0,
            wrong_caps_weight: 1.0,
        };

        let mut rng = thread_rng();
        let system = ErrorSystem::new();
        let error_type = system.select_error_type(&config, &mut rng);
        assert_eq!(error_type, ErrorType::WrongCaps);
    }

    #[test]
    fn test_substitution_uses_adjacent() {
        let mut rng = thread_rng();
        let system = ErrorSystem::new();
        let action = system.generate_error('f', None, ErrorType::Substitution, &mut rng);
        match action {
            ErrorAction::Substitute { intended, actual } => {
                assert_eq!(intended, 'f');
                // 'f' neighbors: r, t, d, g, c, v
                let neighbors = ['r', 't', 'd', 'g', 'c', 'v'];
                assert!(neighbors.contains(&actual), "Got '{}' which is not adjacent to 'f'", actual);
            }
            _ => panic!("Expected Substitute"),
        }
    }

    #[test]
    fn test_correction_sequence() {
        let mut rng = thread_rng();
        let system = ErrorSystem::new();
        let error = ErrorAction::Substitute { intended: 'h', actual: 'j' };
        let config = CorrectionConfig {
            enabled: true,
            speed: CorrectionSpeed::Quick,
            over_backspace_chance: 0.0, // no over-backspace for predictable test
            custom_correction_delay: None,
        };
        let steps = system.generate_correction(&error, &config, &mut rng);
        // Should have: Wait (detection) + Backspace + Wait + Wait (pause) + Type('h') + Wait
        assert!(steps.len() >= 4);
        assert!(matches!(steps[0], CorrectionStep::Wait(_)));
        assert!(matches!(steps[1], CorrectionStep::Backspace));
    }

    #[test]
    fn test_custom_correction_delay() {
        let mut rng = thread_rng();
        let system = ErrorSystem::new();
        let error = ErrorAction::Substitute { intended: 'a', actual: 's' };
        let config = CorrectionConfig {
            enabled: true,
            speed: CorrectionSpeed::Quick, // ignored because custom is set
            over_backspace_chance: 0.0,
            custom_correction_delay: Some((5000, 10000)),
        };
        let steps = system.generate_correction(&error, &config, &mut rng);
        // First step is the detection delay — should be 5000-10000ms
        match &steps[0] {
            CorrectionStep::Wait(d) => {
                assert!(d.as_millis() >= 5000 && d.as_millis() <= 10000);
            }
            _ => panic!("Expected Wait"),
        }
    }

    #[test]
    fn test_very_slow_correction() {
        let speed = CorrectionSpeed::VerySlow;
        let (min, max) = speed.detection_delay_ms();
        assert_eq!(min, 1000);
        assert_eq!(max, 2000);
    }
}
