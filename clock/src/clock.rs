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
    pub show_date: bool,
    pub colon_blink: ColonBlink,
}

fn format_time(time: &DateTime<Local>) -> String {
    time.format("%H:%M:%S").to_string()
}

fn format_date(time: &DateTime<Local>) -> String {
    time.format("%d.%m.%Y").to_string()
}

fn center_line(text: &str, width: usize) -> String {
    let padding = width.saturating_sub(text.chars().count()) / 2;
    format!("{}{text}", " ".repeat(padding))
}

fn colon_visible(time: &DateTime<Local>, colon_blink: ColonBlink) -> bool {
    match colon_blink.state_ms() {
        Some(value) => (time.timestamp_millis() / value as i64) % 2 == 0,
        None => true,
    }
}

fn render_time(time: &DateTime<Local>, pixel_chars: [&str; 2], colon_visible: bool) -> Vec<String> {
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

fn render_date(time: &DateTime<Local>, width: usize) -> String {
    let date_str = format_date(time);
    center_line(&date_str, width)
}

fn lines_width(lines: &[String]) -> usize {
    lines.iter().map(|l| l.chars().count()).max().unwrap_or(0)
}

pub fn render_clock(time: &DateTime<Local>, opts: ClockOptions) -> Vec<String> {
    let pixel_chars = opts.clock_size.pixel_chars();
    let colon_visible = colon_visible(time, opts.colon_blink);

    let mut lines = render_time(time, pixel_chars, colon_visible);

    if opts.show_date {
        let width = lines_width(&lines);
        lines.push(String::new());
        lines.push(render_date(time, width));
    }

    lines
}
