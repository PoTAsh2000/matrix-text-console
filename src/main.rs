mod structs;
mod enums;
mod support;

use crate::support::functions;
use crate::enums::sequence::SequenceType;
use crate::structs::{
    col::ColumnInformation,
    colouredchar::ColouredChar
};

use std::{
    thread,
    time::Duration,
    collections::{ HashMap, VecDeque },
    io:: { self, Write, stdout }
};

use crossterm::{
    execute, cursor,
    cursor::MoveTo,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, size
    }, 
};

use colored::Colorize;

/*
* TODOs / ideas
* Make every character sequence and characters indipendant structs. So they can have their own styling (sequence fading out over time, or white highliting a char that got changed)
*/

fn main() {
    let mut stdout = stdout();

    // Clear the console content
    std::process::Command::new("clear").status().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // Set console size to the maximum terminals size
    let (mut col_count, mut row_count) = size().unwrap();
    print!("\x1b[8;{};{}t", row_count, col_count);
    row_count = row_count - 1;
    stdout.flush().unwrap();

    let mut console_content: Vec<Vec<ColumnInformation>> = Vec::new();
    let mut column_information_map: HashMap<u16, ColumnInformation> = initalize_column_data_map(col_count);

    loop {
        for _r in 0u16..row_count {
            let mut row_content: Vec>Colouted

            for c in 0u16..col_count {
                let mut column_information: ColumnInformation = column_information_map.get(&c)
                    .expect("Column information must exist for column")
                    .clone();

                let character = functions::get_random_character(&column_information);

                let _ = column_information.update_column_information();
                column_information_map.remove(&c);
                column_information_map.insert(c, column_information);
            }
        }
    }
    

    // let mut console_content: Vec<String> = Vec::new();
    // loop {
    //     for _r in 0u16..row_count {
    //         let mut row_buffer: String = "".to_string();

    //         for c in 0u16..col_count {
    //             let mut column_information: ColumnInformation = column_information_map.get(&c)
    //                 .expect("Column information must exist for column")
    //                 .clone();

    //             let character = functions::get_random_character(&column_information);
    //             row_buffer = row_buffer + &character;

    //             let _ = column_information.update_column_information();
    //             column_information_map.remove(&c);
    //             column_information_map.insert(c, column_information);
    //         }


    //         console_content.insert(0, row_buffer);
    //         if console_content.len() > row_count.into() {
    //             let _ = console_content.pop();

    //             execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    //             for (row, row_content) in console_content.iter().enumerate() {
    //                 execute!(stdout, cursor::MoveTo(0, row as u16)).unwrap();
    //                 println!("{}", row_content.green().dimmed());
    //             }
    //             io::stdout().flush().unwrap();
    //             thread::sleep(Duration::from_millis(95));
    //         }
        // }
    // }

    // Ok(())
}

fn initalize_column_data_map(col_count: u16) -> HashMap<u16, ColumnInformation> {
    let mut column_information_map: HashMap<u16, ColumnInformation> = HashMap::new();

    for col_index in 0u16..col_count {
        let starting_type = ColumnInformation::random_sequence_type();

        if starting_type == SequenceType::Whitespace {
            column_information_map.insert(
                col_index.try_into().unwrap(),
                ColumnInformation::new(
                    starting_type,
                    functions::get_rnd_u8_range(4, 10)
                )
            );
        }
        else {
            column_information_map.insert(
                col_index.try_into().unwrap(),
                ColumnInformation::new(
                    starting_type,
                    functions::get_rnd_u8_range(6, 13)
                )
            );
        }
    }

    return column_information_map;
}