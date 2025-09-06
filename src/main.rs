mod structs;
mod support;

use std::{io, thread, time};
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::support::functions;
use structs::col::Col;
use structs::col::WHITESPACE_SEQ_LEN_MIN_MAX;
use structs::col::CHARACTER_SEQ_LEN_MIN_MAX;
use colored::Colorize;

fn main() {
    std::process::Command::new("clear").status().unwrap();
    // Getting teminal data
    let mut row_count: u16 = 7;
    let mut column_count: u16 = 100;
    // if let Some((Width(width), Height(height))) = terminal_size() {
    //     column_count = width;
    //     row_count = height;
    //     println!("Terminal width: {} columns", &width);
    //     println!("Terminal height: {} rows", &height);
    // }

    // Initiating column queues
    let mut column_data_map: HashMap<u16, Col> = HashMap::new();
    for i in 0u16..column_count {
        let col_start_on_witespace_timeout: bool = functions::get_rnd_bool();
        let (seq_range_min, seq_range_max): (u8, u8) = if col_start_on_witespace_timeout { get_whitespace_seq_len() } else { get_character_seq_len() };
        let column_seq_len = functions::get_rnd_u8_range(seq_range_min, seq_range_max);

        let mut column: Col = Col::new(
            column_seq_len,
            col_start_on_witespace_timeout,
        );
        column.initiate_queue(row_count);
        column_data_map.insert(i, column);
    }

    for i in 0..50 {
        std::process::Command::new("clear").status().unwrap();
        let mut terminal_content: String = "".to_string();
        for r in 0u16..row_count {
            for c in 0u16..column_count {
                let mut current_column: Col = column_data_map.get(&c).unwrap().clone();
                
                if let Some(queue_value) = current_column.char_queue.pop_front() {
                    terminal_content = terminal_content + &queue_value;
                }

                current_column.update_column_state();
                current_column.char_queue.push_back(functions::get_random_character(current_column.on_whitespace_timeout));
                column_data_map.remove(&c);
                column_data_map.insert(c, current_column);
            }

            terminal_content = terminal_content + "\n";
        }

        println!("{}", terminal_content.green());

        let sleep_timer = time::Duration::from_millis(2000);
        thread::sleep(sleep_timer);
    }


    let mut stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.clear();
        let _ = stdin.read_line(input);
        println!("{}", input);
    }
}

fn get_whitespace_seq_len() -> (u8, u8) {
    assert!(WHITESPACE_SEQ_LEN_MIN_MAX.len() == 2, "whitespace sequence length min max array must be two long (i1 = min, i2 = max)");
    return (WHITESPACE_SEQ_LEN_MIN_MAX[0], WHITESPACE_SEQ_LEN_MIN_MAX[1]);
}

fn get_character_seq_len() -> (u8, u8) {
    assert!(CHARACTER_SEQ_LEN_MIN_MAX.len() == 2, "character sequence length min max array must be two long (i1 = min, i2 = max)");
    return (CHARACTER_SEQ_LEN_MIN_MAX[0], CHARACTER_SEQ_LEN_MIN_MAX[1]);
}