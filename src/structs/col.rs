#[path = "../support/functions.rs"]
mod support;
use crate::functions;
use std::collections::VecDeque;

pub static WHITESPACE_SEQ_LEN_MIN_MAX: [u8; 2] = [3, 6];
pub static CHARACTER_SEQ_LEN_MIN_MAX: [u8; 2] = [6, 13];

pub struct Col {
    pub char_queue: VecDeque<String>,
    pub seq_len: u8,
    pub on_whitespace_timeout: bool,
}

impl Col {
    pub fn new(seq_len: u8, timeout: bool) -> Self {
        Self {
            char_queue: VecDeque::new(),
            seq_len: seq_len,
            on_whitespace_timeout:timeout,
        }
    }

    pub fn initiate_queue(&mut self, row_count: u16) {
        let characters: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789~`'&%".to_string();
        for i in 0..characters.len() {
            let mut random_character = "".to_string();

            let rnd_char_index: usize = functions::get_rnd_u8_range(0, characters.len().try_into().unwrap()) as usize;
            if let Some(rnd_char) = characters.chars().nth(rnd_char_index) {
                random_character = (rnd_char.to_string());
            }

            if self.on_whitespace_timeout {
                random_character = " ".to_string();
            }

            self.update_column_state();
            self.char_queue.push_back(random_character);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.char_queue.is_empty()
    }

    pub fn print_all(&self) {
        if self.char_queue.is_empty() {
            println!("Queue is empty.");
        } 
        else {
            println!("Queue contents:");
            for (i, item) in self.char_queue.iter().enumerate() {
                println!("{}: {}", i, item);
            }
        }
    }

    pub fn update_column_state(&mut self) {
        let before: u8 = self.seq_len;
        self.seq_len = self.seq_len - 1;

        if self.seq_len <= 0 {
            self.switch_timeout();
            self.seq_len = 
                if self.on_whitespace_timeout { functions::get_rnd_u8_range(WHITESPACE_SEQ_LEN_MIN_MAX[0], WHITESPACE_SEQ_LEN_MIN_MAX[1]) } 
                else { functions::get_rnd_u8_range(CHARACTER_SEQ_LEN_MIN_MAX[0], CHARACTER_SEQ_LEN_MIN_MAX[1]) };
        }
    }

    pub fn switch_timeout(&mut self) {
        let before: bool = self.on_whitespace_timeout;
        self.on_whitespace_timeout = !self.on_whitespace_timeout;
    }
}

