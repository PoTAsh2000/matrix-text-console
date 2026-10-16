use std::env;

use crate::ansi::push_decimal;

/// Brightness steps of the trail; fewer steps means fewer redraws.
const LEVEL_STEPS: f32 = 10.0;
/// Steps of the white head highlight.
const WHITE_STEPS: f32 = 4.0;
const HEAD_WHITE: Rgb = Rgb(235, 255, 235);
/// Channel values of the 6x6x6 colour cube of 256-colour terminals.
const CUBE: [u8; 6] = [0, 95, 135, 175, 215, 255];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorMode {
    TrueColor,
    Ansi256,
}

impl ColorMode {
    /// Picks truecolor when the terminal announces it.
    pub fn detect() -> Self {
        let colorterm = env::var("COLORTERM").unwrap_or_default();
        if colorterm == "truecolor" || colorterm == "24bit" || env::var_os("WT_SESSION").is_some() {
            Self::TrueColor
        } else {
            Self::Ansi256
        }
    }

    /// Rounds a colour to one the mode can show exactly.
    pub fn snap(self, color: Rgb) -> Rgb {
        match self {
            Self::TrueColor => color,
            Self::Ansi256 => {
                let (r, g, b) = cube_index(color);
                Rgb(CUBE[r], CUBE[g], CUBE[b])
            }
        }
    }

    /// Appends the foreground colour escape for a snapped colour.
    pub fn push_foreground(self, out: &mut Vec<u8>, color: Rgb) {
        match self {
            Self::TrueColor => {
                out.extend_from_slice(b"\x1b[38;2;");
                push_decimal(out, usize::from(color.0));
                out.push(b';');
                push_decimal(out, usize::from(color.1));
                out.push(b';');
                push_decimal(out, usize::from(color.2));
            }
            Self::Ansi256 => {
                let (r, g, b) = cube_index(color);
                out.extend_from_slice(b"\x1b[38;5;");
                push_decimal(out, 16 + 36 * r + 6 * g + b);
            }
        }
        out.push(b'm');
    }
}

fn cube_index(color: Rgb) -> (usize, usize, usize) {
    let nearest = |value: u8| {
        (0..CUBE.len())
            .min_by_key(|&i| value.abs_diff(CUBE[i]))
            .unwrap_or(0)
    };
    (nearest(color.0), nearest(color.1), nearest(color.2))
}

/// Colour of a cell: green trail at `level`, mixed towards white by `white`.
pub fn shade(level: f32, white: f32) -> Rgb {
    let level = ((level * LEVEL_STEPS).ceil() / LEVEL_STEPS).clamp(0.0, 1.0);
    let white = (white * WHITE_STEPS).round() / WHITE_STEPS;

    let glow = level.powi(3) * 70.0;
    let green = 30.0 + 225.0 * level.powf(1.3);
    let mix = |trail: f32, head: u8| trail + (f32::from(head) - trail) * white;
    Rgb(
        mix(glow, HEAD_WHITE.0) as u8,
        mix(green, HEAD_WHITE.1) as u8,
        mix(glow, HEAD_WHITE.2) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brighter_level_is_greener() {
        assert!(shade(1.0, 0.0).1 > shade(0.5, 0.0).1);
        assert!(shade(0.5, 0.0).1 > shade(0.1, 0.0).1);
    }

    #[test]
    fn white_lights_all_channels() {
        let head = shade(1.0, 1.0);
        assert!(head.0 > 200 && head.2 > 200);
    }

    #[test]
    fn snapping_is_stable() {
        let snapped = ColorMode::Ansi256.snap(shade(0.6, 0.0));
        assert_eq!(ColorMode::Ansi256.snap(snapped), snapped);
    }

    #[test]
    fn foreground_escapes() {
        let mut out = Vec::new();
        ColorMode::TrueColor.push_foreground(&mut out, Rgb(1, 22, 255));
        assert_eq!(out, b"\x1b[38;2;1;22;255m");

        out.clear();
        ColorMode::Ansi256.push_foreground(&mut out, Rgb(0, 255, 0));
        assert_eq!(out, b"\x1b[38;5;46m");
    }
}
