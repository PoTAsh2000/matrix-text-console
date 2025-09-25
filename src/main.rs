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
    collections::HashMap,
    io:: { Write, BufWriter, stdout }
};

use crossterm::{
    execute,
    cursor::Hide,
    terminal::{ Clear, ClearType, EnterAlternateScreen, size }, 
};

use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let mut buf_writer = BufWriter::new(stdout.lock());
    let (_row_count, column_count) = prepare_console();

    let mut column_information_map: HashMap<u16, ColumnInformation> = initalize_column_data_map(column_count);
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
            
            if coloured_character.colour == AllowedColours::BrightGreen { 
                write!(buf_writer, "{}", coloured_character.value.bright_green())?; 
            }
            if coloured_character.colour == AllowedColours::BrightGreenDimmed { 
                write!(buf_writer, "{}", coloured_character.value.bright_green().dimmed())?; 
            }
            if coloured_character.colour == AllowedColours::GreenDimmed { 
                write!(buf_writer, "{}", coloured_character.value.green().dimmed())?; 
            }

            let _ = column_information.update_column_information();
            column_information_map.remove(&c);
            column_information_map.insert(c, column_information);
        }

        buf_writer.flush()?;
        thread::sleep(Duration::from_millis(70));
    }
}

fn prepare_console() -> (u16, u16) {
    let mut stdout = stdout();

    // Clear the console content
    let _ = execute!(stdout, Hide);
    let _ = execute!(stdout, Clear(ClearType::All));
    let _ = execute!(stdout, EnterAlternateScreen).unwrap();

    // Set console size to the maximum terminals size
    let (column_count, mut rows_count) = size().unwrap();

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
            
            continue;
        }
        
        column_information_map.insert(
            col_index.try_into().unwrap(),
            ColumnInformation::new(
                starting_type,
                true,
                functions::get_rnd_u8_range(6, 16)
            )
        );
    }

    return column_information_map;
}