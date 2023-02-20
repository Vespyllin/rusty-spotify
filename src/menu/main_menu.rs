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

    let logo_len: usize = logo.len() - ((9 * 3) + 8);
    let even_space = (0..((WIDTH - logo_len) / 2) - 1)
        .map(|_| ' ')
        .collect::<String>();

    println!("{}{}\n", even_space, logo);

    println!("1. Backup data from Spotify to your file system.");

    println!("2. Recover data from your file system to a Spotify playlist.");

    println!("q. Quit.");

    sep();
}

pub fn handle_input() -> Choice {
    enable_raw_mode().expect("Could not enable raw mode. main_menu::handle_input");
    let choice: Choice;

    match read().expect("failed to read input. main_menu::handle_input") {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => choice = Choice::Exit,

        Event::Key(KeyEvent {
            code: KeyCode::Char(x),
            ..
        }) => match x {
            '1' => choice = Choice::Choice(1),

            '2' => choice = Choice::Choice(2),

            'q' => choice = Choice::Exit,

            _ => choice = Choice::Default,
        },

        _ => choice = Choice::Default,
    }

    // Double input workaround for Windows
    if cfg!(windows) {
        _ = read().expect("failed to read input. main_menu::handle_input");
    }

    disable_raw_mode().expect("Could not disable raw mode. main_menu::handle_input");

    choice
}
