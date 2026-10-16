/// The three greens a character can have.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shade {
    BrightGreen,
    BrightGreenDimmed,
    GreenDimmed,
}

impl Shade {
    /// ANSI escape sequence that selects this shade.
    pub const fn escape(self) -> &'static [u8] {
        match self {
            Self::BrightGreen => b"\x1b[22;92m",
            Self::BrightGreenDimmed => b"\x1b[2;92m",
            Self::GreenDimmed => b"\x1b[2;32m",
        }
    }
}
