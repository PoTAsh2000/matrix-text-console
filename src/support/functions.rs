use rand::Rng;

pub fn get_rnd_u8_range(start: u8, end: u8) -> u8 {
    let mut random = rand::thread_rng();
    return random.gen_range(start..end);
}

pub fn get_rnd_bool() -> bool {
    let random_bit = get_rnd_u8_range(0, 2);
    return random_bit != 0;
}