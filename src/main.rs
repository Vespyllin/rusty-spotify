use backup::pull_albums_data;
use crossterm::event::read;
use std::process::exit;

use menu::{cls, hide_cursor, reset_cursor, reset_terminal, Choice};

pub mod backup;
pub mod menu;
pub mod spotify_client;

enum Mode {
    Backup,
    Recover,
}

fn exit_proc() {
    cls();
    reset_terminal();
    exit(1)
}

fn main_menu() -> Mode {
    loop {
        menu::main_menu::print_menu();

        match menu::main_menu::handle_input() {
            Choice::Choice(x) => {
                match x {
                    1 => return Mode::Backup,
                    2 => return Mode::Recover,
                    _ => panic!("Bad Value received from main menu."),
                };
            }

            Choice::Exit => exit_proc(),

            Choice::Default => reset_cursor(),

            _ => todo!("Add submit to main menu"),
        }
    }
}

fn backup_menu() -> Vec<bool> {
    let mut selection: Vec<bool> = vec![false, false, false];

    loop {
        menu::backup_menu::print_menu(&mut selection);

        match menu::backup_menu::handle_input() {
            Choice::Exit => exit_proc(),
            Choice::Submit => {
                // If at least 1 option is selected, allow the user to continue
                if selection.iter().fold(false, |acc, v| acc || *v) {
                    break;
                }
            }
            Choice::Default => {}
            Choice::Choice(x) => selection[x - 1] = !selection[x - 1],
        }

        reset_cursor();
    }

    selection
}

#[tokio::main]
async fn main() {
    hide_cursor();
    cls();

    // Discard resize event on Windows
    if cfg!(windows) {
        _ = read().unwrap();
    }

    let backup_mode: Mode = main_menu();
    cls();

    let access_token = "";
    match backup_mode {
        Mode::Backup => {
            let selection = backup_menu();
            cls();

            // if selection[0] {}

            // if selection[1] {}

            if selection[2] {
                match pull_albums_data(access_token).await {
                    Ok(x) => {
                        println!("Albums JSON:");
                        println!("{}", x);
                    }
                    Err(x) => {
                        panic!("{}", x);
                    }
                }
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }
    // cls();
}
