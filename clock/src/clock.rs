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

#[derive(Clone, Copy, Default)]
pub enum ClockSize {
    Narrow,
    #[default]
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

#[derive(Clone, Copy, Default)]
pub struct ClockOptions {
    pub size: ClockSize,
}

fn format_time(time: &DateTime<Local>) -> String {
    time.format("%H:%M:%S").to_string()
}

pub fn render_time(time: &DateTime<Local>, pixel_chars: [&str; 2]) -> Vec<String> {
    let time_str = format_time(time);
    let mut lines = Vec::with_capacity(DIGIT_HEIGHT);

    for row in 0..DIGIT_HEIGHT {
        let mut line = String::new();

        for (index, ch) in time_str.chars().enumerate() {
            // Add a space between characters
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

pub fn render_clock(now: &DateTime<Local>, opts: ClockOptions) -> Vec<String> {
    let pixel_chars = opts.size.pixel_chars();

    render_time(&now, pixel_chars)
}
