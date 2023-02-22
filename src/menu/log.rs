use crossterm::style::Stylize;

use crate::menu::WIDTH;

use super::{cls, sep};

pub enum LogType {
    Error(String),
    Completed(String),
    Status(String),
}

fn print(log: &Vec<LogType>) {
    sep();

    let logo = format!("{}", "Status Logs".bold());
    let logo_len: usize = logo.len();
    let even_space = (0..((WIDTH - logo_len + 8) / 2) - 1).map(|_| ' ').collect::<String>();

    println!("{}{}\n", even_space, logo);

    let display_len = 4;

    for msg in &log[(log.len().saturating_sub(display_len))..log.len()] {
        match msg {
            LogType::Status(val) => {
                println!("{}", val);
            }
            LogType::Completed(val) => {
                println!("{}", val.to_owned().green());
            }
            LogType::Error(val) => {
                println!("{}", val.to_owned().red());
            }
        }
    }

    if log.len() < display_len {
        for _ in 0..(display_len - log.len()) {
            println!();
        }
    }
    sep();
}

pub fn push_print(message: LogType, log: &mut Vec<LogType>) {
    log.push(message);
    cls();
    print(log);
}
