use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardRow {
    Number,
    Top,
    Home,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Finger {
    LPinky,
    LRing,
    LMiddle,
    LIndex,
    RIndex,
    RMiddle,
    RRing,
    RPinky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyPosition {
    pub row: KeyboardRow,
    pub col: u8,
    pub finger: Finger,
    pub hand: Hand,
}

impl KeyPosition {
    const fn new(row: KeyboardRow, col: u8, finger: Finger, hand: Hand) -> Self {
        Self { row, col, finger, hand }
    }
}

impl Finger {
    pub fn same_hand(&self, other: &Finger) -> bool {
        self.hand() == other.hand()
    }

    pub fn hand(&self) -> Hand {
        match self {
            Finger::LPinky | Finger::LRing | Finger::LMiddle | Finger::LIndex => Hand::Left,
            Finger::RIndex | Finger::RMiddle | Finger::RRing | Finger::RPinky => Hand::Right,
        }
    }

    pub fn strength_modifier(&self) -> f64 {
        match self {
            Finger::LIndex | Finger::RIndex => 1.0,
            Finger::LMiddle | Finger::RMiddle => 1.05,
            Finger::LRing | Finger::RRing => 1.20,
            Finger::LPinky | Finger::RPinky => 1.28,
        }
    }
}

impl KeyboardRow {
    pub fn modifier(&self) -> f64 {
        match self {
            KeyboardRow::Home => 1.0,
            KeyboardRow::Top => 1.07,
            KeyboardRow::Bottom => 1.15,
            KeyboardRow::Number => 1.32,
        }
    }
}

lazy_static! {
    static ref KEY_MAP: HashMap<char, KeyPosition> = {
        let mut m = HashMap::new();
        use KeyboardRow::*;
        use Finger::*;
        use Hand::*;

        // Number row
        m.insert('`', KeyPosition::new(Number, 0, LPinky, Left));
        m.insert('1', KeyPosition::new(Number, 1, LPinky, Left));
        m.insert('2', KeyPosition::new(Number, 2, LRing, Left));
        m.insert('3', KeyPosition::new(Number, 3, LMiddle, Left));
        m.insert('4', KeyPosition::new(Number, 4, LIndex, Left));
        m.insert('5', KeyPosition::new(Number, 5, LIndex, Left));
        m.insert('6', KeyPosition::new(Number, 6, RIndex, Right));
        m.insert('7', KeyPosition::new(Number, 7, RIndex, Right));
        m.insert('8', KeyPosition::new(Number, 8, RMiddle, Right));
        m.insert('9', KeyPosition::new(Number, 9, RRing, Right));
        m.insert('0', KeyPosition::new(Number, 10, RPinky, Right));
        m.insert('-', KeyPosition::new(Number, 11, RPinky, Right));
        m.insert('=', KeyPosition::new(Number, 12, RPinky, Right));

        // Top row
        m.insert('q', KeyPosition::new(Top, 0, LPinky, Left));
        m.insert('w', KeyPosition::new(Top, 1, LRing, Left));
        m.insert('e', KeyPosition::new(Top, 2, LMiddle, Left));
        m.insert('r', KeyPosition::new(Top, 3, LIndex, Left));
        m.insert('t', KeyPosition::new(Top, 4, LIndex, Left));
        m.insert('y', KeyPosition::new(Top, 5, RIndex, Right));
        m.insert('u', KeyPosition::new(Top, 6, RIndex, Right));
        m.insert('i', KeyPosition::new(Top, 7, RMiddle, Right));
        m.insert('o', KeyPosition::new(Top, 8, RRing, Right));
        m.insert('p', KeyPosition::new(Top, 9, RPinky, Right));
        m.insert('[', KeyPosition::new(Top, 10, RPinky, Right));
        m.insert(']', KeyPosition::new(Top, 11, RPinky, Right));
        m.insert('\\', KeyPosition::new(Top, 12, RPinky, Right));

        // Home row
        m.insert('a', KeyPosition::new(Home, 0, LPinky, Left));
        m.insert('s', KeyPosition::new(Home, 1, LRing, Left));
        m.insert('d', KeyPosition::new(Home, 2, LMiddle, Left));
        m.insert('f', KeyPosition::new(Home, 3, LIndex, Left));
        m.insert('g', KeyPosition::new(Home, 4, LIndex, Left));
        m.insert('h', KeyPosition::new(Home, 5, RIndex, Right));
        m.insert('j', KeyPosition::new(Home, 6, RIndex, Right));
        m.insert('k', KeyPosition::new(Home, 7, RMiddle, Right));
        m.insert('l', KeyPosition::new(Home, 8, RRing, Right));
        m.insert(';', KeyPosition::new(Home, 9, RPinky, Right));
        m.insert('\'', KeyPosition::new(Home, 10, RPinky, Right));

        // Bottom row
        m.insert('z', KeyPosition::new(Bottom, 0, LPinky, Left));
        m.insert('x', KeyPosition::new(Bottom, 1, LRing, Left));
        m.insert('c', KeyPosition::new(Bottom, 2, LMiddle, Left));
        m.insert('v', KeyPosition::new(Bottom, 3, LIndex, Left));
        m.insert('b', KeyPosition::new(Bottom, 4, LIndex, Left));
        m.insert('n', KeyPosition::new(Bottom, 5, RIndex, Right));
        m.insert('m', KeyPosition::new(Bottom, 6, RIndex, Right));
        m.insert(',', KeyPosition::new(Bottom, 7, RMiddle, Right));
        m.insert('.', KeyPosition::new(Bottom, 8, RRing, Right));
        m.insert('/', KeyPosition::new(Bottom, 9, RPinky, Right));

        m
    };

    /// Adjacency map: for each lowercase key, the physically adjacent keys
    static ref ADJACENCY_MAP: HashMap<char, Vec<char>> = {
        let mut m = HashMap::new();

        // Number row
        m.insert('`', vec!['1']);
        m.insert('1', vec!['`', '2', 'q']);
        m.insert('2', vec!['1', '3', 'q', 'w']);
        m.insert('3', vec!['2', '4', 'w', 'e']);
        m.insert('4', vec!['3', '5', 'e', 'r']);
        m.insert('5', vec!['4', '6', 'r', 't']);
        m.insert('6', vec!['5', '7', 't', 'y']);
        m.insert('7', vec!['6', '8', 'y', 'u']);
        m.insert('8', vec!['7', '9', 'u', 'i']);
        m.insert('9', vec!['8', '0', 'i', 'o']);
        m.insert('0', vec!['9', '-', 'o', 'p']);
        m.insert('-', vec!['0', '=', 'p', '[']);
        m.insert('=', vec!['-', '[', ']']);

        // Top row
        m.insert('q', vec!['1', '2', 'w', 'a']);
        m.insert('w', vec!['2', '3', 'q', 'e', 'a', 's']);
        m.insert('e', vec!['3', '4', 'w', 'r', 's', 'd']);
        m.insert('r', vec!['4', '5', 'e', 't', 'd', 'f']);
        m.insert('t', vec!['5', '6', 'r', 'y', 'f', 'g']);
        m.insert('y', vec!['6', '7', 't', 'u', 'g', 'h']);
        m.insert('u', vec!['7', '8', 'y', 'i', 'h', 'j']);
        m.insert('i', vec!['8', '9', 'u', 'o', 'j', 'k']);
        m.insert('o', vec!['9', '0', 'i', 'p', 'k', 'l']);
        m.insert('p', vec!['0', '-', 'o', '[', 'l', ';']);
        m.insert('[', vec!['-', '=', 'p', ']', ';', '\'']);
        m.insert(']', vec!['=', '[', '\\', '\'']);
        m.insert('\\', vec![']']);

        // Home row
        m.insert('a', vec!['q', 'w', 's', 'z']);
        m.insert('s', vec!['w', 'e', 'a', 'd', 'z', 'x']);
        m.insert('d', vec!['e', 'r', 's', 'f', 'x', 'c']);
        m.insert('f', vec!['r', 't', 'd', 'g', 'c', 'v']);
        m.insert('g', vec!['t', 'y', 'f', 'h', 'v', 'b']);
        m.insert('h', vec!['y', 'u', 'g', 'j', 'b', 'n']);
        m.insert('j', vec!['u', 'i', 'h', 'k', 'n', 'm']);
        m.insert('k', vec!['i', 'o', 'j', 'l', 'm', ',']);
        m.insert('l', vec!['o', 'p', 'k', ';', ',', '.']);
        m.insert(';', vec!['p', '[', 'l', '\'', '.', '/']);
        m.insert('\'', vec!['[', ']', ';', '/']);

        // Bottom row
        m.insert('z', vec!['a', 's', 'x']);
        m.insert('x', vec!['s', 'd', 'z', 'c']);
        m.insert('c', vec!['d', 'f', 'x', 'v']);
        m.insert('v', vec!['f', 'g', 'c', 'b']);
        m.insert('b', vec!['g', 'h', 'v', 'n']);
        m.insert('n', vec!['h', 'j', 'b', 'm']);
        m.insert('m', vec!['j', 'k', 'n', ',']);
        m.insert(',', vec!['k', 'l', 'm', '.']);
        m.insert('.', vec!['l', ';', ',', '/']);
        m.insert('/', vec![';', '\'', '.']);

        m
    };
}

pub fn get_key_position(c: char) -> Option<&'static KeyPosition> {
    KEY_MAP.get(&c.to_ascii_lowercase())
}

pub fn get_adjacent_keys(c: char) -> Vec<char> {
    let lower = c.to_ascii_lowercase();
    ADJACENCY_MAP.get(&lower).cloned().unwrap_or_default()
}

/// Returns a danger weighting for how likely a key is to cause typos
pub fn typo_danger_weight(c: char) -> f64 {
    match c.to_ascii_lowercase() {
        's' => 1.4,
        'd' => 1.35,
        'r' | 'e' => 1.3,
        'i' => 1.25,
        'a' | 'n' | 't' | 'o' => 1.15,
        'q' | 'z' | 'x' | 'j' => 0.7,
        _ => 1.0,
    }
}

/// Classify a character for special character modifiers
pub fn char_class_modifier(c: char) -> f64 {
    if c == ' ' {
        return 0.90; // space is fast (thumb)
    }
    if c.is_ascii_lowercase() {
        1.0
    } else if c.is_ascii_uppercase() {
        1.4 // Shift + key
    } else if c.is_ascii_digit() {
        1.32 // number row reach
    } else {
        match c {
            '.' | ',' | '\'' | '"' => 1.15, // common punctuation, pinky
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' => 1.62, // Shift + number
            '(' | ')' => 1.55,
            '[' | ']' | '{' | '}' => 1.50, // brackets
            ':' | ';' => 1.15,
            '-' | '_' | '+' | '=' => 1.35,
            '/' | '\\' | '|' => 1.30,
            '?' => 1.45, // Shift + /
            '~' => 1.60,
            _ => 1.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_positions_exist() {
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert!(get_key_position(c).is_some(), "Missing position for '{}'", c);
        }
    }

    #[test]
    fn test_home_row_modifier() {
        let pos = get_key_position('f').unwrap();
        assert_eq!(pos.row, KeyboardRow::Home);
        assert!((pos.row.modifier() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_adjacent_keys() {
        let adj = get_adjacent_keys('f');
        assert!(adj.contains(&'d'));
        assert!(adj.contains(&'g'));
        assert!(adj.contains(&'r'));
        assert!(adj.contains(&'v'));
    }

    #[test]
    fn test_uppercase_maps_to_lowercase() {
        assert!(get_key_position('A').is_some());
        let a = get_key_position('a').unwrap();
        let big_a = get_key_position('A').unwrap();
        assert_eq!(a.col, big_a.col);
    }
}
