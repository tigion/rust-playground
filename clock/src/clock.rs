// clock.rs

use chrono::{DateTime, Local};

const DIGITS: [[[u8; 3]; 5]; 10] = [
    [[1, 1, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 1, 1]], // 0
    [[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], // 1
    [[1, 1, 1], [0, 0, 1], [1, 1, 1], [1, 0, 0], [1, 1, 1]], // 2
    [[1, 1, 1], [0, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]], // 3
    [[1, 0, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [0, 0, 1]], // 4
    [[1, 1, 1], [1, 0, 0], [1, 1, 1], [0, 0, 1], [1, 1, 1]], // 5
    [[1, 1, 1], [1, 0, 0], [1, 1, 1], [1, 0, 1], [1, 1, 1]], // 6
    [[1, 1, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], // 7
    [[1, 1, 1], [1, 0, 1], [1, 1, 1], [1, 0, 1], [1, 1, 1]], // 8
    [[1, 1, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]], // 9
];

const DIGIT_HEIGHT: usize = DIGITS[0].len();
const DIGIT_WIDTH: usize = DIGITS[0][0].len();

const COLON: [u8; 5] = [0, 1, 0, 1, 0];

const PIXEL_CHARS_NARROW: [&str; 2] = [" ", "█"];
const PIXEL_CHARS_WIDE: [&str; 2] = ["  ", "██"];

#[derive(Clone, Copy)]
pub enum ClockSize {
    Narrow,
    Wide,
}

impl ClockSize {
    pub fn pixel_chars(self) -> [&'static str; 2] {
        match self {
            ClockSize::Narrow => PIXEL_CHARS_NARROW,
            ClockSize::Wide => PIXEL_CHARS_WIDE,
        }
    }
}

pub fn format_time(time: &DateTime<Local>) -> String {
    time.format("%H:%M:%S").to_string()
}

pub fn render_clock(time_str: &str, pixel_chars: [&str; 2]) -> Vec<String> {
    let mut lines = Vec::with_capacity(DIGIT_HEIGHT);

    // for time_char in time_str.chars() {
    for row in 0..DIGIT_HEIGHT {
        let mut line = String::new();

        for (index, ch) in time_str.chars().enumerate() {
            if index > 0 {
                line.push_str(pixel_chars[0]);
            }

            // Add the linewise pixel representation for the character
            if let Some(digit) = ch.to_digit(10) {
                for col in 0..DIGIT_WIDTH {
                    line.push_str(pixel_chars[DIGITS[digit as usize][row][col] as usize]);
                }
            } else if ch == ':' {
                line.push_str(pixel_chars[COLON[row] as usize]);
            } else {
                unreachable!("unsupported character in time string: {ch:?}");
            }
        }

        lines.push(line);
    }

    lines
}
