use crate::{
    ansi,
    palette::{ColorMode, Rgb},
    rain::{Pixel, Rain},
};

const RESET: &[u8] = b"\x1b[0m";

/// Remembers what the terminal shows and writes only the differences.
pub struct Screen {
    width: usize,
    front: Vec<Pixel>,
    mode: ColorMode,
}

impl Screen {
    /// Assumes the terminal is blank.
    pub fn new(width: usize, height: usize, mode: ColorMode) -> Self {
        Self {
            width,
            front: vec![Pixel::BLANK; width * height],
            mode,
        }
    }

    /// Appends the escapes that turn the screen into the current rain.
    pub fn draw(&mut self, rain: &Rain, out: &mut Vec<u8>) {
        let mut cursor: Option<(usize, usize)> = None;
        let mut active: Option<Rgb> = None;

        for y in 0..rain.height() {
            for x in 0..self.width {
                let mut pixel = rain.pixel(x, y);
                if pixel != Pixel::BLANK {
                    pixel.color = self.mode.snap(pixel.color);
                }

                let shown = &mut self.front[y * self.width + x];
                if *shown == pixel {
                    continue;
                }
                *shown = pixel;

                if cursor != Some((x, y)) {
                    ansi::move_to(out, x, y);
                }
                if pixel != Pixel::BLANK && active != Some(pixel.color) {
                    self.mode.push_foreground(out, pixel.color);
                    active = Some(pixel.color);
                }
                out.push(pixel.glyph);
                cursor = Some((x + 1, y));
            }
        }

        if active.is_some() {
            out.extend_from_slice(RESET);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn unchanged_rain_draws_nothing() {
        let rain = Rain::new(40, 20, Rng::from_seed(5));
        let mut screen = Screen::new(40, 20, ColorMode::TrueColor);
        let mut out = Vec::new();
        screen.draw(&rain, &mut out);
        assert_ne!(out.len(), 0);

        out.clear();
        screen.draw(&rain, &mut out);
        assert_eq!(out.len(), 0);
    }

    #[test]
    fn diff_is_smaller_than_full_redraw() {
        let mut rain = Rain::new(100, 40, Rng::from_seed(5));
        let mut screen = Screen::new(100, 40, ColorMode::TrueColor);
        let mut first = Vec::new();
        screen.draw(&rain, &mut first);

        rain.step(0.033);
        let mut next = Vec::new();
        screen.draw(&rain, &mut next);
        assert!(next.len() < first.len());
    }
}
