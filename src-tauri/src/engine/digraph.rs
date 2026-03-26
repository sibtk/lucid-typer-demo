use crate::engine::keyboard_map::{get_key_position, Hand};
use rand::Rng;

/// Calculate the digraph timing modifier for two consecutive characters.
/// Different hands = fastest, same finger = slowest.
pub fn digraph_modifier(prev: char, current: char, rng: &mut impl Rng) -> f64 {
    let prev_pos = match get_key_position(prev) {
        Some(p) => p,
        None => return 1.0,
    };
    let curr_pos = match get_key_position(current) {
        Some(p) => p,
        None => return 1.0,
    };

    // Different hands — fastest
    if prev_pos.hand != curr_pos.hand {
        return rng.gen_range(0.70..=0.85);
    }

    // Same hand, same finger — slowest
    if std::mem::discriminant(&prev_pos.finger) == std::mem::discriminant(&curr_pos.finger) {
        return rng.gen_range(1.40..=1.80);
    }

    // Same hand, different fingers — check adjacency
    let finger_distance = (prev_pos.col as i8 - curr_pos.col as i8).unsigned_abs();
    if finger_distance <= 1 {
        // Adjacent fingers
        rng.gen_range(1.10..=1.25)
    } else {
        // Non-adjacent fingers on same hand
        rng.gen_range(1.00..=1.15)
    }
}

/// Check if a digraph pair uses different hands (for rollover typing)
pub fn is_different_hands(prev: char, current: char) -> bool {
    let prev_pos = get_key_position(prev);
    let curr_pos = get_key_position(current);
    match (prev_pos, curr_pos) {
        (Some(p), Some(c)) => p.hand != c.hand,
        _ => false,
    }
}

/// Known fast digraphs (different hands, high frequency)
pub fn is_fast_digraph(prev: char, current: char) -> bool {
    let pair = (prev.to_ascii_lowercase(), current.to_ascii_lowercase());
    matches!(
        pair,
        ('t', 'h') | ('h', 'e') | ('a', 'n') | ('r', 'e') | ('e', 'r') |
        ('i', 'n') | ('o', 'n') | ('a', 't') | ('e', 'n') | ('e', 's') |
        ('o', 'r') | ('t', 'i') | ('t', 'e') | ('i', 's') | ('i', 't') |
        ('a', 'l') | ('a', 'r') | ('o', 'u') | ('o', 'f') | ('t', 'o') |
        ('h', 'a') | ('n', 'g') | ('n', 't') | ('s', 't')
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_different_hands_faster() {
        let mut rng = thread_rng();
        // "th" uses different hands (t=left, h=right)
        let mod_th = digraph_modifier('t', 'h', &mut rng);
        assert!(mod_th < 1.0, "Different hands should be < 1.0, got {}", mod_th);
    }

    #[test]
    fn test_same_finger_slower() {
        let mut rng = thread_rng();
        // "ed" uses same finger (both middle left)
        let mod_ed = digraph_modifier('e', 'd', &mut rng);
        // e = LMiddle, d = LMiddle → same finger
        assert!(mod_ed > 1.0, "Same finger should be > 1.0, got {}", mod_ed);
    }

    #[test]
    fn test_is_different_hands() {
        assert!(is_different_hands('t', 'h'));
        assert!(!is_different_hands('e', 'd'));
    }
}
