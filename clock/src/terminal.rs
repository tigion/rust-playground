// terminal.rs

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let guard = Self;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

pub fn draw_centered(out: &mut impl Write, lines: &[String]) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;

    let height = lines.len() as u16;
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as u16;

    let x = cols.saturating_sub(width) / 2;
    let y = rows.saturating_sub(height) / 2;

    for (i, line) in lines.iter().enumerate() {
        queue!(out, MoveTo(x, y + i as u16), Print(line))?;
    }
    out.flush()
}
