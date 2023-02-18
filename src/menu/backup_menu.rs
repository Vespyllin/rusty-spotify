use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::menu::{sep, WIDTH};

use super::Choice;

pub fn print_menu(selected: &mut Vec<bool>) {
    sep();

    let prompt = "Select which option/s to export to JSON.";
    let prompt_len: usize = prompt.len() - 9;

    let even_space = (0..((WIDTH - prompt_len) / 2) - 1)
        .map(|_| ' ')
        .collect::<String>();

    println!("{}{}\n", even_space, prompt.bold());

    let options = vec!["Liked tracks.", "Saved playlists.", "Saved Albums."];

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
            let prompt = format!("{}. {}", (i + 1), option);
            if *highlight {
                println!("{}", prompt.black().on_green());
            } else {
                println!("{}", prompt);
            }
        });

    sep();
}

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

        _ => choice = Choice::Default,
    }

    // Double input workaround for Windows
    if cfg!(windows) {
        _ = read().unwrap();
    }

    disable_raw_mode().unwrap();

    choice
}
