#![allow(unused_imports)]

use std::{
    io::{stdout, Write},
    process::exit,
};

use crossterm::{
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use menu::reset_terminal;

use crate::menu::{cls, Choice};

pub mod menu;

fn exit_proc() {
    cls();
    reset_terminal();
    exit(1)
}

fn selection_menu_handler() {
    let mut selection = vec![false, false, false];
    loop {
        menu::selection_menu::print_menu(&mut selection);

        match menu::selection_menu::handle_input() {
            Choice::Exit => exit_proc(),
            Choice::Submit => break,
            Choice::Default => {}
            Choice::Choice(x) => selection[x - 1] = !selection[x - 1],
        }

        cls();
    }
}

fn main() {
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
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
