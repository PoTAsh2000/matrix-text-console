use crate::rng::Rng;

/// What a column currently emits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SequenceKind {
    Glyphs,
    Gap,
}

const GLYPH_LENGTH: (u8, u8) = (6, 16);
const GAP_LENGTH: (u8, u8) = (6, 18);

/// State of one falling column.
#[derive(Clone, Debug)]
pub struct Column {
    pub kind: SequenceKind,
    /// True for the first cell of a glyph sequence.
    pub sequence_start: bool,
    remaining: u8,
}

impl Column {
    pub fn random(rng: &mut Rng) -> Self {
        if rng.range(0, 2) == 0 {
            Self::glyphs(true, rng)
        } else {
            Self::gap(rng)
        }
    }

    fn glyphs(sequence_start: bool, rng: &mut Rng) -> Self {
        Self {
            kind: SequenceKind::Glyphs,
            sequence_start,
            remaining: rng.range(GLYPH_LENGTH.0, GLYPH_LENGTH.1),
        }
    }

    fn gap(rng: &mut Rng) -> Self {
        Self {
            kind: SequenceKind::Gap,
            sequence_start: false,
            remaining: rng.range(GAP_LENGTH.0, GAP_LENGTH.1),
        }
    }

    /// Moves one cell down and switches sequence when it ends.
    pub fn advance(&mut self, rng: &mut Rng) {
        self.remaining -= 1;
        self.sequence_start = false;

        if self.remaining == 0 {
            *self = match self.kind {
                SequenceKind::Gap => Self::glyphs(true, rng),
                SequenceKind::Glyphs => Self::gap(rng),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequences_alternate() {
        let mut rng = Rng::from_seed(7);
        let mut column = Column::gap(&mut rng);
        let mut starts = 0;
        for _ in 0..200 {
            column.advance(&mut rng);
            if column.sequence_start {
                starts += 1;
                assert_eq!(column.kind, SequenceKind::Glyphs);
            }
        }
        assert!(starts > 0);
    }

    #[test]
    fn lengths_stay_in_range() {
        let mut rng = Rng::from_seed(7);
        for _ in 0..500 {
            let column = Column::random(&mut rng);
            let (min, max) = match column.kind {
                SequenceKind::Glyphs => GLYPH_LENGTH,
                SequenceKind::Gap => GAP_LENGTH,
            };
            assert!((min..max).contains(&column.remaining));
        }
    }
}
