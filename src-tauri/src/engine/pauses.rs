use std::time::Duration;
use rand::Rng;

use crate::engine::config::{ParagraphPauseLevel, ThinkingPauseConfig};

/// Generate a paragraph pause duration (Rule 9)
pub fn paragraph_pause(level: &ParagraphPauseLevel, rng: &mut impl Rng) -> Duration {
    let (min_ms, max_ms) = level.range_ms();
    if min_ms == 0 && max_ms == 0 {
        return Duration::ZERO;
    }
    Duration::from_millis(rng.gen_range(min_ms..=max_ms))
}

/// Determine if a thinking pause should occur at this position (Rule 10)
pub fn should_think(config: &ThinkingPauseConfig, rng: &mut impl Rng) -> bool {
    if !config.enabled {
        return false;
    }
    rng.gen::<f64>() < config.frequency
}

/// Generate a thinking pause duration (Rule 10)
pub fn thinking_pause(config: &ThinkingPauseConfig, rng: &mut impl Rng) -> Duration {
    let (min_ms, max_ms) = config.preset.range_ms();
    Duration::from_millis(rng.gen_range(min_ms..=max_ms))
}

/// Check if a character is a sentence-ending punctuation (thinking pause trigger)
pub fn is_sentence_end(c: char) -> bool {
    matches!(c, '.' | '?' | '!')
}

/// Check if a character is a clause boundary (thinking pause trigger)
pub fn is_clause_boundary(c: char) -> bool {
    matches!(c, ',' | ';' | ':' | '\u{2014}') // includes em dash
}

/// Check if we're at a paragraph break (double newline)
pub fn is_paragraph_break(text: &[char], pos: usize) -> bool {
    if pos + 1 < text.len() {
        text[pos] == '\n' && text[pos + 1] == '\n'
    } else {
        false
    }
}

/// Parse inline pause commands from text
/// Returns the command and how many characters it consumed
pub fn parse_inline_command(text: &str, pos: usize) -> Option<(InlineCommand, usize)> {
    let remaining = &text[pos..];

    if remaining.starts_with("[PAUSE:") {
        if let Some(end) = remaining.find(']') {
            let num_str = &remaining[7..end];
            if let Ok(ms) = num_str.parse::<u64>() {
                return Some((InlineCommand::Pause(Duration::from_millis(ms)), end + 1));
            }
        }
    } else if remaining.starts_with("[THINK]") {
        return Some((InlineCommand::Think, 7));
    } else if remaining.starts_with("[LONG_PAUSE]") {
        return Some((InlineCommand::LongPause, 12));
    }

    None
}

#[derive(Debug, Clone)]
pub enum InlineCommand {
    Pause(Duration),
    Think,
    LongPause,
}

/// Preprocess text into segments, separating inline commands from text
pub fn preprocess_text(text: &str) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut pos = 0;
    let bytes = text.as_bytes();

    while pos < text.len() {
        if bytes[pos] == b'[' {
            if let Some((cmd, consumed)) = parse_inline_command(text, pos) {
                // Flush accumulated text
                if !current_text.is_empty() {
                    segments.push(TextSegment::Text(std::mem::take(&mut current_text)));
                }
                match cmd {
                    InlineCommand::Pause(d) => segments.push(TextSegment::Pause(d)),
                    InlineCommand::Think => segments.push(TextSegment::Think),
                    InlineCommand::LongPause => segments.push(TextSegment::LongPause),
                }
                pos += consumed;
                continue;
            }
        }
        current_text.push(text.as_bytes()[pos] as char);
        pos += 1;
    }

    if !current_text.is_empty() {
        segments.push(TextSegment::Text(current_text));
    }

    segments
}

#[derive(Debug, Clone)]
pub enum TextSegment {
    Text(String),
    Pause(Duration),
    Think,
    LongPause,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use crate::engine::config::ThinkingPausePreset;

    #[test]
    fn test_paragraph_pause_none() {
        let mut rng = thread_rng();
        let d = paragraph_pause(&ParagraphPauseLevel::None, &mut rng);
        assert_eq!(d, Duration::ZERO);
    }

    #[test]
    fn test_paragraph_pause_normal() {
        let mut rng = thread_rng();
        let d = paragraph_pause(&ParagraphPauseLevel::Normal, &mut rng);
        assert!(d.as_millis() >= 5000 && d.as_millis() <= 15000);
    }

    #[test]
    fn test_preprocess_text() {
        let text = "Hello [PAUSE:2000] world [THINK] done";
        let segments = preprocess_text(text);
        assert_eq!(segments.len(), 5);
        match &segments[0] {
            TextSegment::Text(t) => assert_eq!(t, "Hello "),
            _ => panic!("Expected text"),
        }
        match &segments[1] {
            TextSegment::Pause(d) => assert_eq!(d.as_millis(), 2000),
            _ => panic!("Expected pause"),
        }
    }

    #[test]
    fn test_sentence_end_detection() {
        assert!(is_sentence_end('.'));
        assert!(is_sentence_end('?'));
        assert!(!is_sentence_end(','));
    }
}
