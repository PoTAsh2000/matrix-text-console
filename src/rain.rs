use crate::{
    column::Column,
    palette::{self, Rgb},
    rng::Rng,
};

const GLYPHS: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}[]()#$%^&?<>";
/// Cells dimmer than this are blank.
const MIN_LEVEL: f32 = 0.04;
/// Chance per column and step that one glyph in the trail changes.
const MUTATION_CHANCE: f32 = 0.04;
/// Steps run at start so the screen is already filled.
const WARM_UP_STEPS: usize = 600;
const WARM_UP_SECONDS: f32 = 1.0 / 30.0;

#[derive(Clone, Copy)]
struct Cell {
    glyph: u8,
    /// Trail brightness, `0.0..=1.0`.
    level: f32,
    decay: f32,
    /// Head highlight, `0.0..=1.0`.
    white: f32,
}

const BLANK: Cell = Cell {
    glyph: b' ',
    level: 0.0,
    decay: 0.0,
    white: 0.0,
};

/// What one screen cell looks like.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel {
    pub glyph: u8,
    pub color: Rgb,
}

impl Pixel {
    pub const BLANK: Self = Self {
        glyph: b' ',
        color: Rgb(0, 0, 0),
    };
}

/// The rain: one stream per terminal column over a grid of cells.
pub struct Rain {
    width: usize,
    height: usize,
    columns: Vec<Column>,
    cells: Vec<Cell>,
    rng: Rng,
}

impl Rain {
    pub fn new(width: u16, height: u16, rng: Rng) -> Self {
        let mut rain = Self {
            width: usize::from(width),
            height: usize::from(height),
            columns: Vec::new(),
            cells: Vec::new(),
            rng,
        };
        rain.reset();
        rain
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Starts over for a new terminal size.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = usize::from(width);
        self.height = usize::from(height);
        self.reset();
    }

    fn reset(&mut self) {
        self.cells = vec![BLANK; self.width * self.height];
        self.columns = (0..self.width)
            .map(|_| Column::new(&mut self.rng))
            .collect();
        for _ in 0..WARM_UP_STEPS {
            self.step(WARM_UP_SECONDS);
        }
    }

    /// Look of the cell at `x`, `y`.
    pub fn pixel(&self, x: usize, y: usize) -> Pixel {
        let cell = self.cells[y * self.width + x];
        if cell.level < MIN_LEVEL && cell.white <= 0.0 {
            return Pixel::BLANK;
        }
        Pixel {
            glyph: cell.glyph,
            color: palette::shade(cell.level, cell.white),
        }
    }

    /// Advances the simulation by `seconds`.
    pub fn step(&mut self, seconds: f32) {
        for cell in &mut self.cells {
            cell.level = (cell.level - cell.decay * seconds).max(0.0);
            cell.white = 0.0;
        }

        for x in 0..self.width {
            self.columns[x].advance(seconds, self.height, &mut self.rng);
            self.spawn_glyphs(x);
            self.light_head(x);
            self.mutate_trail(x);
        }
    }

    fn cell_mut(&mut self, x: usize, row: i32) -> Option<&mut Cell> {
        let y = usize::try_from(row).ok().filter(|&y| y < self.height)?;
        Some(&mut self.cells[y * self.width + x])
    }

    /// Places a glyph in every row the head has reached.
    fn spawn_glyphs(&mut self, x: usize) {
        let lead = self.columns[x].lead_row();
        while self.columns[x].spawned <= lead {
            self.columns[x].spawned += 1;
            let row = self.columns[x].spawned;
            let glyph = self.random_glyph();
            let decay = self.columns[x].decay();
            if let Some(cell) = self.cell_mut(x, row) {
                *cell = Cell {
                    glyph,
                    level: 0.0,
                    decay,
                    white: 0.0,
                };
            }
        }
    }

    /// Shares the head over the row it is in and the row below it.
    fn light_head(&mut self, x: usize) {
        let lead = self.columns[x].lead_row();
        let fraction = self.columns[x].fraction();

        if let Some(cell) = self.cell_mut(x, lead) {
            cell.level = 1.0;
            cell.white = 1.0 - fraction;
        }
        if let Some(cell) = self.cell_mut(x, lead + 1) {
            cell.level = cell.level.max(fraction);
            cell.white = fraction;
        }
    }

    /// Now and then swaps one glyph behind the head.
    fn mutate_trail(&mut self, x: usize) {
        if self.rng.unit() >= MUTATION_CHANCE {
            return;
        }
        let trail = self.columns[x].trail_rows().max(1) as u32;
        let row = self.columns[x].lead_row() - 1 - self.rng.below(trail) as i32;
        let glyph = self.random_glyph();
        if let Some(cell) = self.cell_mut(x, row)
            && cell.level > 0.2
        {
            cell.glyph = glyph;
        }
    }

    fn random_glyph(&mut self) -> u8 {
        GLYPHS[self.rng.below(GLYPHS.len() as u32) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit_cells(rain: &Rain) -> usize {
        (0..rain.height())
            .flat_map(|y| (0..rain.width()).map(move |x| (x, y)))
            .filter(|&(x, y)| rain.pixel(x, y) != Pixel::BLANK)
            .count()
    }

    #[test]
    fn warm_up_fills_part_of_the_screen() {
        let rain = Rain::new(80, 30, Rng::from_seed(1));
        let lit = lit_cells(&rain);
        assert!(lit > 0 && lit < 80 * 30);
    }

    #[test]
    fn rain_keeps_moving() {
        let mut rain = Rain::new(80, 30, Rng::from_seed(1));
        let before: Vec<_> = (0..30).map(|y| rain.pixel(5, y)).collect();
        for _ in 0..30 {
            rain.step(0.033);
        }
        let after: Vec<_> = (0..30).map(|y| rain.pixel(5, y)).collect();
        assert_ne!(before, after);
    }

    #[test]
    fn head_is_whiter_than_trail() {
        let mut rain = Rain::new(40, 30, Rng::from_seed(2));
        for _ in 0..50 {
            rain.step(0.033);
            for x in 0..40 {
                let head = rain.columns[x].lead_row();
                if (0..30).contains(&head) {
                    let head_pixel = rain.pixel(x, head as usize);
                    assert!(head_pixel.color.0 > 0, "head must carry white");
                }
            }
        }
    }

    #[test]
    fn handles_tiny_and_empty_sizes() {
        for (width, height) in [(0, 0), (1, 1), (3, 0), (0, 5)] {
            let mut rain = Rain::new(width, height, Rng::from_seed(1));
            rain.step(0.033);
            rain.resize(2, 2);
            rain.step(0.033);
        }
    }

    #[test]
    fn resize_changes_size() {
        let mut rain = Rain::new(10, 10, Rng::from_seed(1));
        rain.resize(25, 12);
        assert_eq!((rain.width(), rain.height()), (25, 12));
    }
}
