use crate::rng::Rng;

/// Fall speed in rows per second.
const SPEED: (f32, f32) = (7.0, 14.0);
/// Length of the trail in rows.
const TRAIL: (f32, f32) = (6.0, 16.0);
/// Empty rows between two streams.
const GAP: (f32, f32) = (6.0, 18.0);

/// One falling stream: a moving head with a fading trail behind it.
#[derive(Clone, Debug)]
pub struct Column {
    /// Row position of the head; negative while waiting to start.
    head: f32,
    speed: f32,
    trail: f32,
    /// Last row that received a glyph.
    pub spawned: i32,
}

impl Column {
    pub fn new(rng: &mut Rng) -> Self {
        let mut column = Self {
            head: 0.0,
            speed: 0.0,
            trail: 0.0,
            spawned: 0,
        };
        column.restart(rng);
        column
    }

    fn restart(&mut self, rng: &mut Rng) {
        self.speed = rng.range_f32(SPEED.0, SPEED.1);
        self.trail = rng.range_f32(TRAIL.0, TRAIL.1);
        self.head = -rng.range_f32(GAP.0, GAP.1);
        self.spawned = self.lead_row();
    }

    /// Brightness lost per second by the glyphs of this stream.
    pub fn decay(&self) -> f32 {
        self.speed / self.trail
    }

    pub fn trail_rows(&self) -> i32 {
        self.trail as i32
    }

    /// Row the head is in.
    pub fn lead_row(&self) -> i32 {
        self.head.floor() as i32
    }

    /// How far the head is into its row, `0.0..1.0`.
    pub fn fraction(&self) -> f32 {
        self.head - self.head.floor()
    }

    /// Moves the head; starts a new stream when the trail has left the screen.
    pub fn advance(&mut self, seconds: f32, height: usize, rng: &mut Rng) {
        self.head += self.speed * seconds;
        if self.head - self.trail > height as f32 {
            self.restart(rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stream_waits_above_screen() {
        let mut rng = Rng::from_seed(3);
        for _ in 0..200 {
            let column = Column::new(&mut rng);
            assert!(column.lead_row() < 0);
            assert!((SPEED.0..SPEED.1).contains(&column.speed));
        }
    }

    #[test]
    fn restarts_after_leaving_screen() {
        let mut rng = Rng::from_seed(3);
        let mut column = Column::new(&mut rng);
        let mut restarts = 0;
        for _ in 0..2000 {
            let before = column.head;
            column.advance(0.033, 20, &mut rng);
            if column.head < before {
                restarts += 1;
            }
        }
        assert!(restarts > 1);
    }

    #[test]
    fn head_never_outruns_max_speed() {
        let mut rng = Rng::from_seed(3);
        let mut column = Column::new(&mut rng);
        let before = column.head;
        column.advance(1.0, 1000, &mut rng);
        assert!(column.head - before <= SPEED.1);
    }
}
