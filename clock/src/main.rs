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

fn run_clock(opts: ClockOptions) -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    let mut out = io::stdout();

    loop {
        let now = Local::now();

        let (cols, _) = terminal::size()?;
        let frame_opts = ClockOptions {
            // FIX: Hardcoded threshold for switching to narrow mode
            size: if cols < 54 {
                ClockSize::Narrow
            } else {
                opts.size
            },
            ..opts
        };

        let rendered_clock = render_clock(&now, frame_opts);
        draw_centered(&mut out, &rendered_clock)?;

        let ms = u64::from(now.timestamp_subsec_millis() % 1000);
        if event::poll(Duration::from_millis(1000 - ms))? {
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
    let opts = ClockOptions {
        size: ClockSize::Wide,
        show_date: true,
    };

    if std::env::args().any(|arg| arg == "--print") {
        print_clock(opts);
    } else {
        run_clock(opts)?;
    }

    Ok(())
}
