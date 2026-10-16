use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    palette::ColorMode,
    rain::Rain,
    rng::Rng,
    screen::Screen,
    terminal::{self, TerminalGuard},
};

/// Time between two frames. Fall speed is set per column, not by this.
const FRAME_INTERVAL: Duration = Duration::from_millis(33);

const BEGIN_SYNC: &[u8] = b"\x1b[?2026h";
const END_SYNC: &[u8] = b"\x1b[?2026l";

pub fn run() -> io::Result<()> {
    let _terminal = TerminalGuard::enter()?;
    let mode = ColorMode::detect();
    let (width, height) = TerminalGuard::size()?;
    let mut rain = Rain::new(width, height, Rng::from_time());
    let mut screen = Screen::new(rain.width(), rain.height(), mode);
    let mut out = stdout().lock();
    let mut frame = Vec::with_capacity(rain.width() * rain.height());
    let mut next_frame = Instant::now();

    loop {
        frame.clear();
        frame.extend_from_slice(BEGIN_SYNC);
        screen.draw(&rain, &mut frame);
        frame.extend_from_slice(END_SYNC);
        terminal::present(&mut out, &frame)?;

        rain.step(FRAME_INTERVAL.as_secs_f32());

        // Fixed deadline keeps the speed free of drift
        next_frame += FRAME_INTERVAL;
        match wait_for_next_frame(next_frame)? {
            Input::Quit => return Ok(()),
            Input::Resize(width, height) => {
                rain.resize(width, height);
                screen = Screen::new(rain.width(), rain.height(), mode);
                TerminalGuard::clear()?;
            }
            Input::None => {}
        }
        // Skip missed frames instead of catching up
        next_frame = next_frame.max(Instant::now());
    }
}

enum Input {
    None,
    Quit,
    Resize(u16, u16),
}

/// Handles input until the deadline passes.
fn wait_for_next_frame(deadline: Instant) -> io::Result<Input> {
    let mut resize = None;
    while event::poll(deadline.saturating_duration_since(Instant::now()))? {
        match event::read()? {
            Event::Key(key) if is_quit(key) => return Ok(Input::Quit),
            Event::Resize(width, height) => resize = Some((width, height)),
            _ => {}
        }
    }
    Ok(resize.map_or(Input::None, |(width, height)| Input::Resize(width, height)))
}

fn is_quit(key: KeyEvent) -> bool {
    key.kind == KeyEventKind::Press
        && match key.code {
            KeyCode::Esc | KeyCode::Char('q') => true,
            KeyCode::Char('c') => key.modifiers.contains(KeyModifiers::CONTROL),
            _ => false,
        }
}
