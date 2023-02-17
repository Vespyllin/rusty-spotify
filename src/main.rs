use std::process::exit;

use menu::{cls, hide_cursor, reset_terminal, Choice};

pub mod menu;

fn exit_proc() {
    cls();
    reset_terminal();
    exit(1)
}

fn selection_menu_handler() {
    let mut selection: Vec<bool> = vec![false, false, false];
    loop {
        menu::selection_menu::print_menu(&mut selection);

        match menu::selection_menu::handle_input() {
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

        cls();
    }
}

fn main() {
    hide_cursor();
    cls();

    loop {
        menu::main_menu::print_menu();
        match menu::main_menu::handle_input() {
            Choice::Choice(x) => {
                cls();
                match x {
                    1 => selection_menu_handler(),
                    2 => todo!("Export"),
                    _ => todo!("Bad Value"),
                };

                break;
            }

            Choice::Exit => exit_proc(),
            Choice::Default => cls(),
            _ => todo!("Add submit to main menu"),
        }
    }

    cls();
}
