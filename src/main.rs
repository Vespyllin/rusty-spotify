use std::process::exit;

use crossterm::event::read;
use menu::{cls, hide_cursor, reset_cursor, reset_terminal, Choice};

pub mod menu;

enum Mode {
    Backup,
    Recover,
}

fn exit_proc() {
    cls();
    reset_terminal();
    exit(1)
}

fn backup_selection_handler() -> Vec<bool> {
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

fn main() {
    hide_cursor();
    cls();

    let backup_mode: Mode;

    // Discard resize event on Windows
    if cfg!(windows) {
        _ = read().unwrap();
    }

    loop {
        menu::main_menu::print_menu();

        match menu::main_menu::handle_input() {
            Choice::Choice(x) => {
                match x {
                    1 => backup_mode = Mode::Backup,
                    2 => backup_mode = Mode::Recover,
                    _ => panic!("Bad Value received from main menu."),
                };

                break;
            }

            Choice::Exit => exit_proc(),

            Choice::Default => reset_cursor(),

            _ => todo!("Add submit to main menu"),
        }
    }
    cls();

    match backup_mode {
        Mode::Backup => {
            let selection = backup_selection_handler();

            if selection[0] {
                todo!("liked pipeline")
            }
            if selection[1] {
                todo!("playlist pipeline")
            }
            if selection[2] {
                todo!("albums pipeline")
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }
    cls();
}
