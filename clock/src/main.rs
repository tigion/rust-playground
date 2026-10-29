// main.rs

mod clock;
mod terminal;

use chrono::Local;
use clock::{ClockSize, format_time, render_clock};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{Clear, ClearType},
};
// use std::io;
// use std::time::Duration;
use std::{io, time::Duration};

use crate::terminal::{TerminalGuard, draw, draw_centered};

fn print_clock(size: ClockSize) {
    let time_string = format_time(&Local::now());
    let lines = render_clock(&time_string, size.pixel_chars());

    println!("{}", lines.join("\n"));
}

fn run_clock(size: ClockSize) -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    let pixel_chars = size.pixel_chars();
    let mut out = io::stdout();

    loop {
        let now = Local::now();
        let time_string = format_time(&now);
        let rendered_clock = render_clock(&time_string, pixel_chars);
        draw_centered(&mut out, &rendered_clock)?;
        // draw(&mut out, &rendered_clock)?;

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
    let size = ClockSize::Wide;

    if std::env::args().any(|arg| arg == "--print") {
        print_clock(size);
    } else {
        run_clock(size)?;
    }

    Ok(())
}
