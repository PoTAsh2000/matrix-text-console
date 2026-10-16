use crate::{
    column::{Column, SequenceKind},
    palette::Shade,
    rng::Rng,
};

const GLYPHS: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}[]()#$%^&?<>";
const RESET: &[u8] = b"\x1b[0m";

/// The rain: one `Column` per terminal column.
pub struct Rain {
    columns: Vec<Column>,
    rng: Rng,
}

impl Rain {
    pub fn new(width: u16, mut rng: Rng) -> Self {
        let columns = (0..width).map(|_| Column::random(&mut rng)).collect();
        Self { columns, rng }
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }

    /// Adds or drops columns to match the new terminal width.
    pub fn resize(&mut self, width: u16) {
        let rng = &mut self.rng;
        self.columns
            .resize_with(usize::from(width), || Column::random(rng));
    }

    /// Appends one new top row to `out` and advances every column.
    pub fn write_row(&mut self, out: &mut Vec<u8>) {
        let mut active: Option<Shade> = None;

        for column in &mut self.columns {
            if column.kind == SequenceKind::Gap {
                out.push(b' ');
            } else {
                let shade = pick_shade(column.sequence_start, &mut self.rng);
                if active != Some(shade) {
                    out.extend_from_slice(shade.escape());
                    active = Some(shade);
                }
                out.push(GLYPHS[self.rng.below(GLYPHS.len() as u32) as usize]);
            }
            column.advance(&mut self.rng);
        }

        out.extend_from_slice(RESET);
    }
}

fn pick_shade(sequence_start: bool, rng: &mut Rng) -> Shade {
    let chance = rng.below(100);
    match (sequence_start, chance) {
        (true, 0..60) => Shade::BrightGreen,
        (true, _) | (false, 0..40) => Shade::BrightGreenDimmed,
        (false, _) => Shade::GreenDimmed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell_count(row: &[u8]) -> usize {
        // Count bytes outside escape sequences
        let mut count = 0;
        let mut in_escape = false;
        for &byte in row {
            match (in_escape, byte) {
                (false, 0x1b) => in_escape = true,
                (true, b'm') => in_escape = false,
                (false, _) => count += 1,
                (true, _) => {}
            }
        }
        count
    }

    #[test]
    fn row_has_one_cell_per_column() {
        let mut rain = Rain::new(120, Rng::from_seed(1));
        let mut out = Vec::new();
        for _ in 0..50 {
            out.clear();
            rain.write_row(&mut out);
            assert_eq!(cell_count(&out), 120);
        }
    }

    #[test]
    fn resize_changes_width() {
        let mut rain = Rain::new(10, Rng::from_seed(1));
        rain.resize(25);
        assert_eq!(rain.width(), 25);
        rain.resize(5);
        assert_eq!(rain.width(), 5);
    }
}
