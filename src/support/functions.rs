
use crate::enums::sequence::SequenceType;
use crate::structs::col::ColumnInformation;
use rand::Rng;

pub fn get_rnd_u8_range(start: u8, end: u8) -> u8 {
    let mut random = rand::rng();
    return random.random_range(start..end);
}

pub fn get_rnd_bool() -> bool {
    let random_bit = get_rnd_u8_range(0, 2);
    return random_bit != 0;
}

pub fn get_random_character(column_information: &ColumnInformation) -> String {
    if column_information.sequence_type == SequenceType::Whitespace {
        return " ".to_string();
    }

    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}[]()#$%^&?<>".to_string();
    let rnd_char_index: usize = get_rnd_u8_range(
        0,
        characters.len().try_into().unwrap()
    ) as usize;

    return characters
        .chars()
        .nth(rnd_char_index)
        .expect("Must be created")
        .to_string();
}