use crossterm::style::Stylize;

use crate::menu::{cls, handle_input, sep, teardown, Choice, MENU_WIDTH};

#[derive(PartialEq)]
pub enum MenuOptions {
    Backup,
    Restore,
}

pub fn menu() -> MenuOptions {
    cls();
    sep();

    let logo = format!(
        "{} {} {}",
        "Welcome to loveless'".bold(),
        "Spotify".green().bold(),
        "backup tool!".bold()
    );

    let logo_len: usize = logo.len() - ((9 * 3) + 8);
    let even_space = (0..((MENU_WIDTH - logo_len) / 2) - 1)
        .map(|_| ' ')
        .collect::<String>();

    println!("{}{}\n", even_space, logo);

    println!("1. Backup data from Spotify to your file system.");

    println!("2. Recover data from your file system to a Spotify playlist.");

    println!();

    println!("q. Quit.");

    println!();

    sep();

    match handle_input() {
        Choice::Exit => teardown(),

        Choice::Option(key) => match key {
            '1' => MenuOptions::Backup,
            '2' => MenuOptions::Restore,
            _ => menu(),
        },

        _ => menu(),
    }
}
