// main.rs

mod clock;
mod screen;

use chrono::Local;
use clock::{ClockOptions, ClockSize, render_clock};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use screen::{TerminalGuard, draw_centered};
use std::{io, time::Duration};

/// Prints the clock once to the standard output.
fn print_clock(opts: ClockOptions) {
    let now = Local::now();
    let lines = render_clock(&now, opts);

    println!("{}", lines.join("\n"));
}

/// Determines the largest clock size that fits within the available width.
///
/// The selected size never exceeds the configured clock size.
fn fit_clock_size(cols: u16, opts: ClockOptions) -> ClockSize {
    // TODO: Better fit the clock size to the terminal dimensions:
    //       - check also the terminal height (rows)
    //       - handle if the smallest clock size doesn't fit

    let cols = usize::from(cols);
    let fitted = if cols < clock::clock_size_width(ClockSize::Narrow, opts.show_seconds) {
        ClockSize::Text
    } else if cols < clock::clock_size_width(ClockSize::Wide, opts.show_seconds) {
        ClockSize::Narrow
    } else {
        ClockSize::Wide
    };
    fitted.min(opts.clock_size)
}

/// Runs the interactive clock display.
///
/// Continuously renders the clock centered in the terminal and handles
/// keyboard input and terminal resizing.
///
/// # Errors
///
/// Returns an error if terminal operations or event handling fail.
fn run_clock(opts: ClockOptions) -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    let mut out = io::stdout();

    loop {
        let now = Local::now();

        let (cols, _) = terminal::size()?;
        let frame_opts = ClockOptions {
            clock_size: fit_clock_size(cols, opts),
            ..opts
        };

        let rendered_clock = render_clock(&now, frame_opts);
        draw_centered(&mut out, &rendered_clock)?;

        // Align the next frame with the blink interval.
        let frame_interval = opts.colon_blink.frame_ms();
        let ms = u64::from(now.timestamp_subsec_millis()) % frame_interval;
        if event::poll(Duration::from_millis(frame_interval - ms))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    let ctrl_c = key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL);
                    if ctrl_c || matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                        break;
                    }
                }
                Event::Resize(..) => execute!(out, Clear(ClearType::All))?,
                _ => {}
            }
        }
    }

    Ok(())
}

/// Initializes the clock and starts the selected display mode.
///
/// Uses `--print` to print the clock once instead of starting the
/// interactive display.
///
/// # Errors
///
/// Returns an error if terminal initialization or rendering fails.
fn main() -> io::Result<()> {
    let mut opts = ClockOptions::default();

    // TODO: Add command-line arguments for:
    //       - displaying seconds
    //       - selecting the clock size
    //       - configuring the colon blink interval

    // opts.show_seconds = true;
    // opts.colon_blink = clock::ColonBlink::Off;
    // opts.colon_blink = clock::ColonBlink::Second;

    if std::env::args().any(|arg| arg == "--print") {
        opts.colon_blink = clock::ColonBlink::Off;
        print_clock(opts);
    } else {
        run_clock(opts)?;
    }

    Ok(())
}
