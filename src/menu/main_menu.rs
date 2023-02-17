use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::menu::{sep, WIDTH};

use super::Choice;

pub fn print_menu() {
    sep();

    let logo = format!(
        "{} {} {}",
        "Welcome to loveless'".bold(),
        "Spotify".green().bold(),
        "backup tool!".bold()
    );
    let logo_len: u16 = (logo.len() - ((9 * 3) + 8)).try_into().unwrap();

    let even_space = (0..((WIDTH - logo_len) / 2) - 1)
        .map(|_| ' ')
        .collect::<String>();

    println!("={}{}{}=", even_space, logo, even_space);

    let underscore = (0..WIDTH - 2).map(|_| ' ').collect::<String>();
    println!("={}=", underscore);

    let prompt = "1. Backup data from Spotify to your file system.";
    let prompt_len: u16 = prompt.len().try_into().unwrap();

    let space = (0..(WIDTH - (prompt_len + 9)))
        .map(|_| ' ')
        .collect::<String>();

    println!("=\t{}{}=", prompt, space);

    let prompt = "2. Export data from your file system to a Spotify playlist.";
    let prompt_len: u16 = prompt.len().try_into().unwrap();

    let space = (0..(WIDTH - (prompt_len + 9)))
        .map(|_| ' ')
        .collect::<String>();

    println!("=\t{}{}=", prompt, space);

    let prompt = "q. Quit.";
    let prompt_len: u16 = prompt.len().try_into().unwrap();

    let space = (0..(WIDTH - (prompt_len + 9)))
        .map(|_| ' ')
        .collect::<String>();

    println!("=\t{}{}=", prompt, space);

    sep();
}

pub fn handle_input() -> Choice {
    enable_raw_mode().unwrap();
    let choice: Choice;

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
            code: KeyCode::Char('q'),
            ..
        }) => choice = Choice::Exit,

        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => choice = Choice::Exit,

        _ => choice = Choice::Default,
    }

    disable_raw_mode().unwrap();

    choice
}
