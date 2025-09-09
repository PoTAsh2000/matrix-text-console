use crate::enums::sequence::SequenceType;
use crate::support::functions;

#[derive(Clone)]
pub struct ColumnInformation {
    pub sequence_type: SequenceType,
    pub swap_timer: u8,
}

impl ColumnInformation {
    pub fn new(initial_sequence_type: SequenceType, intial_swap_timer: u8) -> Self {
        Self {
            sequence_type: initial_sequence_type,
            swap_timer: intial_swap_timer,
        }
    }


    pub fn random_sequence_type() -> SequenceType {
        let random_value: u8 = functions::get_rnd_u8_range(0, 2);
        if random_value == 0 {
            return SequenceType::Character;
        }
        return SequenceType::Whitespace;
    }


    pub fn update_column_information(&mut self) {
        self.swap_timer = self.swap_timer - 1;

        if self.swap_timer == 0 {
            if self.sequence_type == SequenceType::Whitespace {
                self.sequence_type = SequenceType::Character;
                self.swap_timer = functions::get_rnd_u8_range(6, 13);
            }
            else {
                self.sequence_type = SequenceType::Whitespace;
                self.swap_timer = functions::get_rnd_u8_range(4, 10);
            }
        }
    }
}

