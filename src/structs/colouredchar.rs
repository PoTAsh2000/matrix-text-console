use crate::enums::allowedcolours::AllowedColours;

#[derive(Clone)]
pub struct ColouredChar {
    pub value: String,
    pub colour: AllowedColours,
}

impl ColouredChar {
    pub fn new(value: String, colour: AllowedColours) -> Self {
        Self {
            value: value,
            colour: colour,
        }
    }
}