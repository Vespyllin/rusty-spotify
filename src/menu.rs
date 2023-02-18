use std::io::{stdout, Write};

use crossterm::{
    cursor, execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};

pub mod backup_menu;
pub mod main_menu;

pub enum Choice {
    Choice(usize),
    Exit,
    Submit,
    Default,
}

pub const WIDTH: usize = 85;
pub const FILLER_CHAR: char = '=';

pub fn reset_cursor() {
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
}

pub fn cls() {
    if cfg!(windows) {
        queue!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        queue!(stdout(), Print("\x1B[2J\x1B[1;1H")).unwrap();
        queue!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        stdout().flush().unwrap();
    } else {
        execute!(stdout(), Print("\x1B[2J\x1B[1;1H")).unwrap();
    }
}

pub fn sep() {
    let sep = (0..WIDTH).map(|_| FILLER_CHAR).collect::<String>();
    println!("{}", sep);
}

pub fn reset_terminal() {
    if !is_raw_mode_enabled().unwrap() {
        enable_raw_mode().unwrap();
    }

    execute!(stdout(), crossterm::style::ResetColor).expect("Failed to reset terminal colors!");
    execute!(stdout(), crossterm::cursor::Show).expect("Failed to reset cursor status!");
    execute!(stdout(), crossterm::cursor::MoveLeft(100)).expect("Failed to reset cursor position!");

    disable_raw_mode().unwrap();
}

pub fn hide_cursor() {
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
}
