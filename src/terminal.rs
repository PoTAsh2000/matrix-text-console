use std::{
    io::{self, Write, stdout},
    panic,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

/// Asks the terminal to maximize its window.
const REQUEST_FULLSCREEN: &[u8] = b"\x1b[9;1t";
/// Longest wait for the window to finish resizing.
const FULLSCREEN_TIMEOUT: Duration = Duration::from_millis(500);
/// Quiet time after the last resize event.
const RESIZE_SETTLE: Duration = Duration::from_millis(100);

/// Owns the terminal state; restores it on drop.
pub struct TerminalGuard;

impl TerminalGuard {
    /// Goes fullscreen, then enters raw mode and the alternate screen.
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        // Restore the terminal before a panic message is printed
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            restore();
            default_hook(info);
        }));

        let guard = Self;
        request_fullscreen()?;
        execute!(stdout(), EnterAlternateScreen, Hide, Clear(ClearType::All))?;
        Ok(guard)
    }

    /// Current terminal size as width and height.
    pub fn size() -> io::Result<(u16, u16)> {
        size()
    }

    /// Clears the screen, used after a resize.
    pub fn clear() -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All))
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore();
    }
}

fn restore() {
    let _ = execute!(stdout(), Show, LeaveAlternateScreen);
    let _ = disable_raw_mode();
}

/// Maximizes the window and waits until the new size is stable.
fn request_fullscreen() -> io::Result<()> {
    let mut out = stdout();
    out.write_all(REQUEST_FULLSCREEN)?;
    out.flush()?;

    let deadline = Instant::now() + FULLSCREEN_TIMEOUT;
    let mut wait = FULLSCREEN_TIMEOUT;
    while event::poll(wait)? {
        if matches!(event::read()?, Event::Resize(..)) {
            wait = RESIZE_SETTLE;
        }
        wait = wait.min(deadline.saturating_duration_since(Instant::now()));
    }
    Ok(())
}

/// Writes a finished frame to the terminal in one call.
pub fn present(out: &mut impl Write, frame: &[u8]) -> io::Result<()> {
    out.write_all(frame)?;
    out.flush()
}
