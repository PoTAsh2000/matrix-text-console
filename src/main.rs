mod structs;
mod enums;
mod support;

use support::functions;
use enums::sequence::SequenceType;
use structs::col::ColumnInformation;
use std::collections::HashMap;
use std::io::{self, Write, stdout};
use std::{thread, time::Duration};
use crossterm::{
    execute,
    cursor,
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};
use colored::{Colorize, Color};

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    std::process::Command::new("clear").status().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let (mut col_count, mut row_count) = size().unwrap();
    print!("\x1b[8;{};{}t", row_count, col_count);
    row_count = row_count - 1;
    stdout.flush().unwrap();

    let mut column_information_map: HashMap<u16, ColumnInformation> = initalize_column_data_map(col_count);

    let mut console_content: Vec<String> = Vec::new();
    loop {
        for _r in 0u16..row_count {
            let mut row_buffer: String = "".to_string();

            for c in 0u16..col_count {
                let mut column_information: ColumnInformation = column_information_map.get(&c)
                    .expect("Column information must exist for column")
                    .clone();

                let character = functions::get_random_character(&column_information);
                row_buffer = row_buffer + &character;

                let _ = column_information.update_column_information();
                column_information_map.remove(&c);
                column_information_map.insert(c, column_information);
            }


            console_content.insert(0, row_buffer);
            if console_content.len() > row_count.into() {
                let _ = console_content.pop();

                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                for (row, row_content) in console_content.iter().enumerate() {
                    execute!(stdout, cursor::MoveTo(0, row as u16)).unwrap();
                    println!("{}", row_content.green().dimmed());
                }
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(95));
            }
        }
    }

    Ok(())
}


// fn get_character_color() -> Color {
//     let rnd: u8 = functions::get_rnd_u8_range(0, 11);
//     if rnd < 10 {
//         return Color::Green;
//     }
//     return Color::BrightGreen;
// }


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