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

const COLON_WIDTH: usize = 1;

const PIXEL_CHARS_NARROW: [&str; 2] = [" ", "█"];
const PIXEL_CHARS_WIDE: [&str; 2] = ["  ", "██"];

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum ClockSize {
    Text,
    Narrow,
    #[default]
    Wide,
}

impl ClockSize {
    pub fn pixel_chars(self) -> Option<[&'static str; 2]> {
        match self {
            ClockSize::Text => None,
            ClockSize::Narrow => Some(PIXEL_CHARS_NARROW),
            ClockSize::Wide => Some(PIXEL_CHARS_WIDE),
        }
    }
}

/// Blink behavior of the colon separators.
#[derive(Clone, Copy, Default)]
pub enum ColonBlink {
    /// The colon is always visible.
    Off,
    /// 0.5 s visible, 0.5 s hidden.
    #[default]
    HalfSecond,
    /// 1 s visible, 1 s hidden.
    Second,
}

impl ColonBlink {
    /// Duration of one visible/hidden state in milliseconds, `None` if the colon does not blink.
    pub const fn state_ms(self) -> Option<u64> {
        match self {
            ColonBlink::Off => None,
            ColonBlink::HalfSecond => Some(500),
            ColonBlink::Second => Some(1000),
        }
    }

    /// How often the display must be redrawn, in milliseconds.
    pub const fn frame_ms(self) -> u64 {
        match self {
            ColonBlink::Off | ColonBlink::Second => 1000,
            ColonBlink::HalfSecond => 500,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct ClockOptions {
    pub clock_size: ClockSize,
    pub colon_blink: ColonBlink,
    pub show_seconds: bool,
}

fn format_time(time: &DateTime<Local>, show_seconds: bool) -> String {
    let fmt = if show_seconds { "%H:%M:%S" } else { "%H:%M" };
    time.format(fmt).to_string()
}

fn colon_visible(time: &DateTime<Local>, colon_blink: ColonBlink) -> bool {
    match colon_blink.state_ms() {
        Some(value) => (time.timestamp_millis() / value as i64) % 2 == 0,
        None => true,
    }
}

fn render_pixel_time(
    time: &DateTime<Local>,
    pixel_chars: [&str; 2],
    colon_visible: bool,
    show_seconds: bool,
) -> Vec<String> {
    let time_str = format_time(time, show_seconds);
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
                let idx = if colon_visible { COLON[row] } else { 0 };
                line.push_str(pixel_chars[idx as usize]);
            } else {
                unreachable!("unsupported character in time string: {ch:?}");
            }
        }

        lines.push(line);
    }

    lines
}

fn render_text_time(time: &DateTime<Local>, colon_visible: bool, show_seconds: bool) -> String {
    let text = format_time(time, show_seconds);
    if colon_visible {
        text
    } else {
        text.replace(':', " ")
    }
}

fn render_time(
    time: &DateTime<Local>,
    size: ClockSize,
    colon_visible: bool,
    show_seconds: bool,
) -> Vec<String> {
    match size.pixel_chars() {
        Some(pixel_chars) => render_pixel_time(time, pixel_chars, colon_visible, show_seconds),
        None => vec![render_text_time(time, colon_visible, show_seconds)],
    }
}

// TODO: Find a better way to calculate the width
pub fn clock_size_width(size: ClockSize, show_seconds: bool) -> usize {
    let (num_digits, num_colons) = if show_seconds { (6, 2) } else { (4, 1) }; // HH:MM:SS or HH:MM
    match size.pixel_chars() {
        Some(pixel_chars) => {
            let pixel_width = pixel_chars[1].chars().count();
            num_digits * (DIGIT_WIDTH * pixel_width)
                + num_colons * (COLON_WIDTH * pixel_width)
                + (num_digits + num_colons - 1) * pixel_chars[0].chars().count() // spaces between characters
        }
        None => num_digits + num_colons, // text mode, each character is 1 column wide
    }
}

pub fn render_clock(time: &DateTime<Local>, opts: ClockOptions) -> Vec<String> {
    let colon_visible = colon_visible(time, opts.colon_blink);
    let lines = render_time(time, opts.clock_size, colon_visible, opts.show_seconds);

    lines
}
