use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::menu::sep;

use super::Choice;

pub fn print_menu(selected: &mut Vec<bool>) {
    sep();

    println!("Select which option/s to export to JSON.\n");

    let options = vec![
        "1.\tExport liked tracks.",
        "2.\tExport saved playlists.",
        "3.\tExport saved Albums.",
    ];

    for (i, option) in options.iter().enumerate() {
        let highlight = *(selected.get(i).unwrap_or(&false));

        if highlight {
            println!("{}", option.on_dark_green())
        } else {
            println!("{}", option)
        }
    }

    sep();
}

// let mut selection = vec![false, false, false]
pub fn handle_input() -> Choice {
    let choice: Choice;

    enable_raw_mode().unwrap();

    match read().unwrap() {
        Event::Key(KeyEvent {
            code: KeyCode::Char('1'),
            ..
        }) => {
            choice = Choice::Choice(1);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('2'),
            ..
        }) => {
            choice = Choice::Choice(2);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('3'),
            ..
        }) => {
            choice = Choice::Choice(3);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) => choice = Choice::Exit,

        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => choice = Choice::Exit,

        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => choice = Choice::Submit,

        _ => {
            choice = Choice::Default;
        }
    }

    disable_raw_mode().unwrap();

    choice
}
