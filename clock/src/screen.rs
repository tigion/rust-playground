// screen.rs

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

/// Manages terminal initialization and cleanup.
///
/// Enters raw mode and the alternate screen when created.
/// Restores the terminal state when dropped.
pub struct TerminalGuard;

impl TerminalGuard {
    /// Initializes the terminal for content rendering.
    ///
    /// Enables raw mode, enters the alternate screen, and hides the cursor.
    ///
    /// # Errors
    ///
    /// Returns an error if terminal initialization fails.
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let guard = Self;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    /// Restores the terminal state.
    ///
    /// Shows the cursor, leaves the alternate screen, and disables raw mode.
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

/// Returns the width of the longest line in characters.
fn lines_width(lines: &[String]) -> usize {
    lines.iter().map(|l| l.chars().count()).max().unwrap_or(0)
}

/// Draws the given lines centered in the terminal.
///
/// # Arguments
///
/// * `out` - The output stream to draw to.
/// * `lines` - The lines to display.
///
/// # Errors
///
/// Returns an error if the terminal size cannot be retrieved or writing to the
/// output stream fails.
pub fn draw_centered(out: &mut impl Write, lines: &[String]) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;

    let height = lines.len() as u16; // <= rows, fits into u16
    let width = lines_width(lines) as u16; // <= cols, fits into u16

    let x = cols.saturating_sub(width) / 2;
    let y = rows.saturating_sub(height) / 2;

    for (row, line) in (y..).zip(lines) {
        queue!(out, MoveTo(x, row), Print(line))?;
    }

    out.flush()
}
