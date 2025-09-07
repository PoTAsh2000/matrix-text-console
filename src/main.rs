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
    
    // Setting the amount for rows and columns
    let row_count: u16 = 70;
    let column_count: u16 =209;

    // Initiating column data
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

    // row manager to update and keep track of the content for each row 
    let mut row_manager: VecDeque<String> = VecDeque::new();

    for _i in 0..50 {
        for r in 0u16..row_count {
            // Before processing a new row, clear the terminal content
            std::process::Command::new("clear").status().unwrap();

            // Create row content and update column states
            let mut row_content: String = "".to_string();
            for c in 0u16..column_count {
                // Create row content
                let mut current_column: Col = column_data_map.get(&c).unwrap().clone();
                let mut new_char_queue: VecDeque<String> = VecDeque::new(); // Create a new queue in case a character in the queue has changed

                if let Some(queue_value) = current_column.char_queue.pop_front() {
                    let before: String = queue_value.clone();
                    let mut character_to_display: String = queue_value;

                    let rnd_number = functions::get_rnd_u8_range(0, 4);
                    if (rnd_number == 0) && (!current_column.on_whitespace_timeout) {
                        character_to_display = functions::get_random_character(current_column.on_whitespace_timeout);
                        new_char_queue.push_back(character_to_display.clone());
                    }

                    current_column.char_queue = new_char_queue;
                    row_content = row_content + &character_to_display;
                }

                // Update column state
                current_column.update_column_state();
                current_column.char_queue.push_back(functions::get_random_character(current_column.on_whitespace_timeout));
                column_data_map.remove(&c);
                column_data_map.insert(c, current_column);
            }
            row_manager.push_front(row_content);

            if row_manager.len() >= row_count.into() {
                let mut matrix_console: String = "".to_string();
                for (_i, item) in row_manager.iter().enumerate() {
                    matrix_console = matrix_console + item + "\n";
                }
                println!("{}", matrix_console.green());

                let sleep_timer = time::Duration::from_millis(250);
                thread::sleep(sleep_timer);

                row_manager.pop_back();
            }
        }
    }

    // Keep the console open
    let stdin = io::stdin();
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