use rand::Rng;

pub fn get_rnd_u8_range(start: u8, end: u8) -> u8 {
    let mut random = rand::rng();
    return random.random_range(start..end);
}

pub fn get_rnd_bool() -> bool {
    let random_bit = get_rnd_u8_range(0, 2);
    return random_bit != 0;
}

pub fn get_random_character (on_whitespace_timeout: bool) -> String {
    let characters: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789~`'&%".to_string();
    
    let mut random_character = "".to_string();
    let rnd_char_index: usize = get_rnd_u8_range(0, characters.len().try_into().unwrap()) as usize;
    if let Some(rnd_char) = characters.chars().nth(rnd_char_index) {
        random_character = rnd_char.to_string();
    }

    if on_whitespace_timeout {
        random_character = " ".to_string();
    }

    return random_character;
}