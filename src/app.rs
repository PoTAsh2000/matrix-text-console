use std::{
    io::{self, stdout},
    thread,
    time::{Duration, Instant},
};

use crate::{rain::Rain, rng::Rng, terminal};

/// Time between two rows; sets the fall speed.
const FRAME_INTERVAL: Duration = Duration::from_millis(70);

const BEGIN_SYNC: &[u8] = b"\x1b[?2026h";
const END_SYNC: &[u8] = b"\x1b[?2026l";
const SCROLL_DOWN: &[u8] = b"\x1b[H\x1b[1L";

pub fn run() -> io::Result<()> {
    let width = terminal::prepare()?;
    let mut rain = Rain::new(width, Rng::from_time());
    let mut out = stdout().lock();
    let mut frame = Vec::with_capacity(usize::from(width) * 16);
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
        let now = Instant::now();
        if next_frame > now {
            thread::sleep(next_frame - now);
        } else {
            next_frame = now;
        }
    }
}
