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

fn print_clock(opts: ClockOptions) {
    let now = Local::now();
    let lines = render_clock(&now, opts);

    println!("{}", lines.join("\n"));
}

// TODO: Needs optimization
fn fit_clock_size(cols: u16, opts: ClockOptions) -> ClockSize {
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

fn main() -> io::Result<()> {
    let mut opts = ClockOptions::default();

    // opts.blink_interval = ColonBlink::Second;

    if std::env::args().any(|arg| arg == "--print") {
        opts.colon_blink = clock::ColonBlink::Off;
        print_clock(opts);
    } else {
        run_clock(opts)?;
    }

    Ok(())
}
