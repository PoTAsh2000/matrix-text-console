use std::io::{self, Write};

use crossterm::{
    cursor::Hide,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, size},
};

/// Prepares the screen and returns the terminal width.
pub fn prepare() -> io::Result<u16> {
    let mut out = io::stdout();
    execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;
    let (width, _height) = size()?;
    Ok(width)
}

/// Writes a finished frame to the terminal in one call.
pub fn present(out: &mut impl Write, frame: &[u8]) -> io::Result<()> {
    out.write_all(frame)?;
    out.flush()
}
