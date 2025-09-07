#[path = "../support/functions.rs"]
mod support;
use crate::functions;
use std::collections::VecDeque;

pub static WHITESPACE_SEQ_LEN_MIN_MAX: [u8; 2] = [5, 10];
pub static CHARACTER_SEQ_LEN_MIN_MAX: [u8; 2] = [8, 12];

#[derive(Clone)]
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
        for _i in 0..row_count {
            let random_character = functions::get_random_character(self.on_whitespace_timeout);

            self.update_column_state();

            self.char_queue.push_back(random_character);
        }
    }

    pub fn print_all(&self, row: u16, column: u16) {
        let mut queue_values: String = "".to_string();
        for (_i, item) in self.char_queue.iter().enumerate() {
            queue_values = queue_values + item;
        }
        println!("queue on row[{}], column[{}]: {}",row, column, queue_values);
    }

    pub fn update_column_state(&mut self) {
        self.seq_len = self.seq_len - 1;

        if self.seq_len <= 0 {
            self.switch_timeout();
            self.seq_len = 
                if self.on_whitespace_timeout { functions::get_rnd_u8_range(WHITESPACE_SEQ_LEN_MIN_MAX[0], WHITESPACE_SEQ_LEN_MIN_MAX[1]) } 
                else { functions::get_rnd_u8_range(CHARACTER_SEQ_LEN_MIN_MAX[0], CHARACTER_SEQ_LEN_MIN_MAX[1]) };
        }
    }

    pub fn switch_timeout(&mut self) {
        self.on_whitespace_timeout = !self.on_whitespace_timeout;
    }
}

