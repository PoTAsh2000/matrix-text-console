#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::doc_markdown
)]
//! Frame pipeline benchmark. Run inside a real console window:
//!   perf_bench --fps 30 --secs 8 --out result.txt
//!   perf_bench --fps 0 --delay-ms 2500        (unlimited = throughput ceiling)
//!
//! Metrics: fill (buffer built), flush (host accepted bytes), ack (host parsed
//! bytes, measured with a cursor position request), bytes per frame.

use std::{
    fmt::Write as _,
    io::{self, Write, stdout},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, size},
};
use matrix_text_console::{palette::ColorMode, rain::Rain, rng::Rng, screen::Screen};

const WARM_UP_FRAMES: usize = 6;

fn arg(name: &str, default: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| default.into())
}

fn percentile(values: &mut [f64], p: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(f64::total_cmp);
    values[((values.len() - 1) as f64 * p).round() as usize]
}

fn summary(name: &str, values: &mut [f64], unit: &str) -> String {
    let mean = values.iter().sum::<f64>() / values.len().max(1) as f64;
    format!(
        "{name:<22} mean {mean:>10.1}  p50 {:>10.1}  p95 {:>10.1}  p99 {:>10.1}  max {:>10.1} {unit}\n",
        percentile(values, 0.5),
        percentile(values, 0.95),
        percentile(values, 0.99),
        percentile(values, 1.0),
    )
}

/// Waits for the cursor position reply, which proves the host parsed all bytes.
fn wait_for_reply() -> io::Result<()> {
    while event::poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()?
            && matches!(key.code, KeyCode::Char('R') | KeyCode::F(3))
        {
            break;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let fps: f64 = arg("--fps", "0").parse().expect("--fps");
    let secs: f64 = arg("--secs", "8").parse().expect("--secs");
    let delay_ms: u64 = arg("--delay-ms", "0").parse().expect("--delay-ms");
    let out_path = arg("--out", "perf_result.txt");
    let sync = arg("--sync", "1") == "1";

    thread::sleep(Duration::from_millis(delay_ms));
    let mut out = stdout().lock();
    execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;
    let (cols, rows) = size()?;
    terminal::enable_raw_mode()?;

    let mode = match arg("--color", "auto").as_str() {
        "truecolor" => ColorMode::TrueColor,
        "256" => ColorMode::Ansi256,
        _ => ColorMode::detect(),
    };
    let mut rain = Rain::new(cols, rows, Rng::from_time());
    let mut screen = Screen::new(rain.width(), rain.height(), mode);
    let mut frame = Vec::with_capacity(usize::from(cols) * usize::from(rows));
    let period = (fps > 0.0).then(|| Duration::from_secs_f64(1.0 / fps));
    let mut next_deadline = Instant::now();
    let (mut fill, mut flush, mut ack, mut bytes) = (vec![], vec![], vec![], vec![]);
    let mut missed = 0;
    let mut frame_no = 0;
    let run_start = Instant::now();

    while run_start.elapsed().as_secs_f64() < secs {
        let start = Instant::now();
        frame.clear();
        if sync {
            frame.extend_from_slice(b"\x1b[?2026h");
        }
        rain.step(1.0 / 30.0);
        screen.draw(&rain, &mut frame);
        if sync {
            frame.extend_from_slice(b"\x1b[?2026l");
        }
        let fill_end = Instant::now();
        out.write_all(&frame)?;
        out.flush()?;
        let flush_end = Instant::now();
        out.write_all(b"\x1b[6n")?;
        out.flush()?;
        wait_for_reply()?;
        let end = Instant::now();

        if frame_no >= WARM_UP_FRAMES {
            fill.push((fill_end - start).as_secs_f64() * 1e6);
            flush.push((flush_end - fill_end).as_secs_f64() * 1e6);
            ack.push((end - flush_end).as_secs_f64() * 1e6);
            bytes.push(frame.len() as f64);
            if let Some(p) = period
                && end > next_deadline + p
            {
                missed += 1;
            }
        }
        frame_no += 1;

        if let Some(p) = period {
            next_deadline += p;
            let now = Instant::now();
            if next_deadline > now {
                thread::sleep(next_deadline - now);
            } else if now - next_deadline > p * 4 {
                next_deadline = now;
            }
        }
    }

    terminal::disable_raw_mode()?;
    let total = run_start.elapsed().as_secs_f64();
    let mut report = String::new();
    let _ = writeln!(report, "term={cols}x{rows} fps_target={fps} sync={sync}");
    let _ = writeln!(
        report,
        "frames={frame_no} duration={total:.2}s achieved_fps={:.1} missed_deadlines={missed}",
        frame_no as f64 / total
    );
    report += &summary("bytes/frame", &mut bytes, "B");
    report += &summary("fill (buffer built)", &mut fill, "us");
    report += &summary("flush (write done)", &mut flush, "us");
    report += &summary("ack (host parsed)", &mut ack, "us");
    std::fs::write(out_path, report)
}
