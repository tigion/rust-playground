// clock.rs

use chrono::{DateTime, Local};

/// 5x3 pixel representations of the digits 0–9.
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

/// 5-row pixel representation of the colon.
const COLON: [u8; 5] = [0, 1, 0, 1, 0];

const COLON_WIDTH: usize = 1;

/// Characters used to render narrow pixels: one character per pixel.
const PIXEL_CHARS_NARROW: [&str; 2] = [" ", "█"];

/// Characters used to render wide pixels: two characters per pixel.
const PIXEL_CHARS_WIDE: [&str; 2] = ["  ", "██"];

/// Defines the size of the clock representation.
#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum ClockSize {
    /// Text-based clock representation.
    Text,

    /// Narrow pixel-based clock representation.
    Narrow,

    /// Wide pixel-based clock representation (default).
    #[default]
    Wide,
}

impl ClockSize {
    /// Returns the pixel characters for the selected clock size.
    ///
    /// Returns `None` for text-based clocks.
    pub fn pixel_chars(self) -> Option<[&'static str; 2]> {
        match self {
            ClockSize::Text => None,
            ClockSize::Narrow => Some(PIXEL_CHARS_NARROW),
            ClockSize::Wide => Some(PIXEL_CHARS_WIDE),
        }
    }
}

/// Defines the blinking behavior of the clock's colon.
#[derive(Clone, Copy, Default)]
pub enum ColonBlink {
    /// The colon is always visible.
    Off,

    /// 0.5 seconds visible, 0.5 seconds hidden.
    #[default]
    HalfSecond,

    /// 1 second visible, 1 second hidden.
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

/// Configuration options for rendering a clock.
#[derive(Clone, Copy, Default)]
pub struct ClockOptions {
    /// The size of the clock representation.
    pub clock_size: ClockSize,

    /// The blinking behavior of the clock's colon.
    pub colon_blink: ColonBlink,

    /// Whether seconds are displayed.
    pub show_seconds: bool,
}

/// Formats the given time as HH:MM or HH:MM:SS.
fn format_time(time: &DateTime<Local>, show_seconds: bool) -> String {
    let fmt = if show_seconds { "%H:%M:%S" } else { "%H:%M" };
    time.format(fmt).to_string()
}

/// Determines whether the colon should be visible based on the blink mode.
fn colon_visible(time: &DateTime<Local>, colon_blink: ColonBlink) -> bool {
    match colon_blink.state_ms() {
        Some(value) => (time.timestamp_millis() / value as i64) % 2 == 0,
        None => true,
    }
}

/// Renders the given time using a pixel-based representation.
///
/// Each digit and colon is converted into a multi-line pixel
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

/// Renders the given time as a single-line text representation.
///
/// Replaces hidden colons with spaces.
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

/// Calculates the width of the clock.
///
/// # Arguments
///
/// * `size` - The clock size (Text, Narrow, or Wide).
/// * `show_seconds` - Whether seconds are displayed.
///
/// # Returns
///
/// The width of the clock in characters.
pub fn clock_size_width(size: ClockSize, show_seconds: bool) -> usize {
    // HH:MM:SS or HH:MM
    let (num_digits, num_colons) = if show_seconds { (6, 2) } else { (4, 1) };

    match size.pixel_chars() {
        // Pixel clock: H_H_:_M_M_:_S_S or H_H_:_M_M
        Some(pixel_chars) => {
            let pixel_width = pixel_chars[1].chars().count();

            num_digits * (DIGIT_WIDTH * pixel_width)
                + num_colons * (COLON_WIDTH * pixel_width)
                + (num_digits + num_colons - 1) * pixel_width
        }
        // Text clock: HH:MM:SS or HH:MM
        None => num_digits + num_colons,
    }
}

/// Renders a given time as a clock.
///
/// # Arguments
///
/// * `time` - The time to display.
/// * `opts` - The clock rendering options.
///
/// # Returns
///
/// A vector of strings representing the rendered clock.
pub fn render_clock(time: &DateTime<Local>, opts: ClockOptions) -> Vec<String> {
    let colon_visible = colon_visible(time, opts.colon_blink);

    render_time(time, opts.clock_size, colon_visible, opts.show_seconds)
}
