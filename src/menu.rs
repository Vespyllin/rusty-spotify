use std::{
    io::stdout,
    process::{exit, Command},
};

use crossterm::{
    // cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};

pub enum Choice {
    Option(char),
    Exit,
    Submit,
    Default,
}

pub const MENU_WIDTH: usize = 85;
const FILLER_CHAR: char = '=';

pub fn cls() {
    if cfg!(windows) {
        Command::new("cmd").arg("/C").arg("cls").status().expect("cls failed to run");
    } else {
        execute!(stdout(), Print("\x1B[2J\x1B[1;1H")).expect("cls");
    }
}

pub fn sep() {
    let sep = (0..MENU_WIDTH).map(|_| FILLER_CHAR).collect::<String>();
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

pub fn teardown() -> ! {
    cls();
    reset_terminal();
    exit(1)
}

pub fn setup() {
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
    cls();

    // Discard resize event on Windows
    if cfg!(windows) {
        _ = read().expect("failed to read input. main");
    }
}

pub fn handle_input() -> Choice {
    enable_raw_mode().expect("Could not enable raw mode. main_menu::handle_input");

    let input = read().expect("failed to read input. main_menu::handle_input");
    let choice = match input {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => Choice::Exit,

        Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => Choice::Exit,

        Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => Choice::Submit,

        Event::Key(KeyEvent { code: KeyCode::Char(x), .. }) => Choice::Option(x),

        _ => Choice::Default,
    };

    // Double input workaround for Windows
    if cfg!(windows) {
        _ = read().expect("failed to read input. menu::handle_input");
    }

    disable_raw_mode().expect("Could not disable raw mode. menu::handle_input");

    choice
}
