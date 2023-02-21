use std::{io::stdout, process::Command};

use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};

pub mod backup;
pub mod log;
pub mod main;

pub enum Mode {
    Backup,
    Recover,
}

pub enum Choice {
    Option(usize),
    Exit,
    Submit,
    Default,
}

const WIDTH: usize = 85;
const FILLER_CHAR: char = '=';

pub fn reset_cursor() {
    execute!(stdout(), cursor::MoveTo(0, 0)).expect("Could not reset cursor");
}

pub fn cls() {
    if cfg!(windows) {
        Command::new("cmd")
            .arg("/C")
            .arg("cls")
            .status()
            .expect("cls failed to run");
    } else {
        execute!(stdout(), Print("\x1B[2J\x1B[1;1H")).expect("cls");
    }
}

pub fn sep() {
    let sep = (0..WIDTH).map(|_| FILLER_CHAR).collect::<String>();
    println!("{}", sep);
}

pub fn reset_terminal() {
    if !is_raw_mode_enabled().expect("reset is raw?") {
        enable_raw_mode().expect("reset raw");
    }

    execute!(stdout(), crossterm::style::ResetColor).expect("Failed to reset terminal colors!");
    execute!(stdout(), crossterm::cursor::Show).expect("Failed to reset cursor status!");
    execute!(stdout(), crossterm::cursor::MoveLeft(100)).expect("Failed to reset cursor position!");

    disable_raw_mode().expect("reset raw 2");
}

pub fn hide_cursor() {
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
}
