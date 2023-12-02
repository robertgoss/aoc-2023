fn is_num(ch: char) -> Option<u8> {
    match ch {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}
fn prefix(string: &str) -> Option<u8> {
    if let Some(ch) = string.chars().next() {
        if let Some(val) = is_num(ch) {
            return Some(val);
        }
    }
    let len = string.len();
    if len >= 4 && string[..4] == *"zero" {
        Some(0)
    } else if len >= 3 && string[..3] == *"one" {
        Some(1)
    } else if len >= 3 && string[..3] == *"two" {
        Some(2)
    } else if len >= 5 && string[..5] == *"three" {
        Some(3)
    } else if len >= 4 && string[..4] == *"four" {
        Some(4)
    } else if len >= 4 && string[..4] == *"five" {
        Some(5)
    } else if len >= 3 && string[..3] == *"six" {
        Some(6)
    } else if len >= 5 && string[..5] == *"seven" {
        Some(7)
    } else if len >= 5 && string[..5] == *"eight" {
        Some(8)
    } else if len >= 4 && string[..4] == *"nine" {
        Some(9)
    } else {
        None
    }
}

fn end(string: &str, n: usize) -> &str {
    let i = string.len() - n;
    &string[i..]
}

fn suffix(string: &str) -> Option<u8> {
    if let Some(ch) = string.chars().rev().next() {
        if let Some(val) = is_num(ch) {
            return Some(val);
        }
    }
    let len = string.len();
    if len >= 4 && *end(string, 4) == *"zero" {
        Some(0)
    } else if len >= 3 && *end(string, 3) == *"one" {
        Some(1)
    } else if len >= 3 && *end(string, 3) == *"two" {
        Some(2)
    } else if len >= 5 && *end(string, 5) == *"three" {
        Some(3)
    } else if len >= 4 && *end(string, 4) == *"four" {
        Some(4)
    } else if len >= 4 && *end(string, 4) == *"five" {
        Some(5)
    } else if len >= 3 && *end(string, 3) == *"six" {
        Some(6)
    } else if len >= 5 && *end(string, 5) == *"seven" {
        Some(7)
    } else if len >= 5 && *end(string, 5) == *"eight" {
        Some(8)
    } else if len >= 4 && *end(string, 4) == *"nine" {
        Some(9)
    } else {
        None
    }
}

fn right_value_digits(text: &str) -> u8 {
    text.char_indices()
        .filter_map(|(pos, _)| prefix(&text[pos..]))
        .next()
        .unwrap_or(0)
}

fn left_value_digits(text: &str) -> u8 {
    text.char_indices()
        .rev()
        .filter_map(|(pos, _)| suffix(&text[..=pos]))
        .next()
        .unwrap_or(0)
}

fn right_value(text: &str) -> u8 {
    text.chars().filter_map(is_num).next().unwrap_or(0)
}

fn left_value(text: &str) -> u8 {
    text.chars().rev().filter_map(is_num).next().unwrap_or(0)
}

fn value(text: &str, digits: bool) -> u8 {
    if digits {
        right_value_digits(text) * 10 + left_value_digits(text)
    } else {
        right_value(text) * 10 + left_value(text)
    }
}

pub fn calibration_total(lines: &Vec<String>, digits: bool) -> u32 {
    lines.iter().map(|line| value(line, digits) as u32).sum()
}
