mod structs;
mod enums;
mod support;

use crate::support::functions;
use crate::enums::{
    sequence::SequenceType,
    allowedcolours::AllowedColours
};
use crate::structs::{
    col::ColumnInformation,
    colouredchar::ColouredChar
};

use std::{
    thread,
    time::Duration,
    collections::{ HashMap, VecDeque },
    io:: { self, Write, BufWriter, stdout }
};

use crossterm::{
    execute, cursor,
    cursor::{MoveTo, Hide},
    terminal::{ Clear, ClearType, EnterAlternateScreen, size }, 
};

use colored::{ Colorize, Color };

/*
* TODOs / ideas
* Make every character sequence and characters indipendant structs. So they can have their own styling (sequence fading out over time, or white highliting a char that got changed)
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let mut buf_writer = BufWriter::new(stdout.lock());
    let (row_count, column_count) = prepare_console();

    let mut console_content: Vec<Vec<ColouredChar>> = Vec::new();
    let mut column_information_map: HashMap<u16, ColumnInformation> = initalize_column_data_map(column_count);

    let mut rows_on_screen: u16 = 0;
    let mut characters_on_screen: VecDeque<ColouredChar> = VecDeque::new();
    loop {
        write!(stdout, "\x1b[H").unwrap();
        write!(stdout, "\x1b[1L").unwrap();

        for c in 0u16..column_count {
            let mut column_information: ColumnInformation = column_information_map.get(&c)
                .expect("Column information must exist for column")
                .clone();

            let character = functions::get_random_character(&column_information);
            let character_color = random_character_colour(&column_information);
            let coloured_character = ColouredChar::new(character.clone(), character_color.clone());
            
            if character_color == AllowedColours::BrightGreen { 
                write!(buf_writer, "{}", character.bright_green())?; 
            }
            if character_color == AllowedColours::BrightGreenDimmed { 
                write!(buf_writer, "{}", character.bright_green().dimmed())?; 
            }
            if character_color == AllowedColours::GreenDimmed { 
                write!(buf_writer, "{}", character.green().dimmed())?; 
            }

            let _ = column_information.update_column_information();
            column_information_map.remove(&c);
            column_information_map.insert(c, column_information);
        }

        buf_writer.flush()?;
        thread::sleep(Duration::from_millis(70));
    }

    Ok::<(), Box<dyn std::error::Error>>(());
}

fn prepare_console() -> (u16, u16) {
    let mut stdout = stdout();

    // Clear the console content
    execute!(stdout, Hide);
    execute!(stdout, Clear(ClearType::All));
    execute!(stdout, EnterAlternateScreen).unwrap();

    // Set console size to the maximum terminals size
    let (mut column_count, mut rows_count) = size().unwrap();

    print!("\x1b[8;{};{}t", rows_count, column_count);

    rows_count = rows_count - 1;
    return (rows_count, column_count);
}

fn random_character_colour(column_information: &ColumnInformation) -> AllowedColours {
    let chance_result = functions::get_rnd_u8_range(0, 99);
    if column_information.sequence_started == true {
        if chance_result < 60 {
            return AllowedColours::BrightGreen;
        }
        return AllowedColours::BrightGreenDimmed;
    }

    if chance_result < 40 {
        return AllowedColours::BrightGreenDimmed;
    }
    return AllowedColours::GreenDimmed;
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
                    false,
                    functions::get_rnd_u8_range(6, 18)
                )
            );
        }
        else {
            column_information_map.insert(
                col_index.try_into().unwrap(),
                ColumnInformation::new(
                    starting_type,
                    true,
                    functions::get_rnd_u8_range(6, 16)
                )
            );
        }
    }

    return column_information_map;
}