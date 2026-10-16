use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    rain::Rain,
    rng::Rng,
    terminal::{self, TerminalGuard},
};

/// Time between two rows; sets the fall speed.
const FRAME_INTERVAL: Duration = Duration::from_millis(70);

const BEGIN_SYNC: &[u8] = b"\x1b[?2026h";
const END_SYNC: &[u8] = b"\x1b[?2026l";
const SCROLL_DOWN: &[u8] = b"\x1b[H\x1b[1L";

pub fn run() -> io::Result<()> {
    let _terminal = TerminalGuard::enter()?;
    let mut rain = Rain::new(TerminalGuard::width()?, Rng::from_time());
    let mut out = stdout().lock();
    let mut frame = Vec::with_capacity(rain.width() * 16);
    let mut next_frame = Instant::now();

    loop {
        frame.clear();
        frame.extend_from_slice(BEGIN_SYNC);
        frame.extend_from_slice(SCROLL_DOWN);
        rain.write_row(&mut frame);
        frame.extend_from_slice(END_SYNC);
        terminal::present(&mut out, &frame)?;

        // Fixed deadline keeps the speed free of drift
        next_frame += FRAME_INTERVAL;
        if wait_for_next_frame(next_frame, &mut rain)? == Flow::Quit {
            return Ok(());
        }
        // Skip missed frames instead of catching up
        next_frame = next_frame.max(Instant::now());
    }
}

#[derive(PartialEq, Eq)]
enum Flow {
    Continue,
    Quit,
}

/// Handles input until the deadline passes.
fn wait_for_next_frame(deadline: Instant, rain: &mut Rain) -> io::Result<Flow> {
    while event::poll(deadline.saturating_duration_since(Instant::now()))? {
        match event::read()? {
            Event::Key(key) if is_quit(key) => return Ok(Flow::Quit),
            Event::Resize(width, _height) => {
                rain.resize(width);
                TerminalGuard::clear()?;
            }
            _ => {}
        }
    }
    Ok(Flow::Continue)
}

fn is_quit(key: KeyEvent) -> bool {
    key.kind == KeyEventKind::Press
        && match key.code {
            KeyCode::Esc | KeyCode::Char('q') => true,
            KeyCode::Char('c') => key.modifiers.contains(KeyModifiers::CONTROL),
            _ => false,
        }
}
