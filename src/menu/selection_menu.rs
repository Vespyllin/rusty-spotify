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
        "Export liked tracks.",
        "Export saved playlists.",
        "Export saved Albums.",
    ];

    assert_eq!(
        selected.len(),
        options.len(),
        "Match selected vec len to options len."
    );

    selected
        .iter()
        .zip(options.iter())
        .enumerate()
        .for_each(|(i, (highlight, option))| {
            let num = format!("{}.", i + 1);
            if *highlight {
                println!("{}\t{}", num.on_dark_green(), option)
            } else {
                println!("{}\t{}", num, option)
            }
        });

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

    if cfg!(windows) {
        _ = read().unwrap();
    }

    disable_raw_mode().unwrap();

    choice
}
