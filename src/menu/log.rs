use crossterm::style::Stylize;

use super::cls;

pub enum LogType {
    Error(String),
    Completed(String),
    Status(String),
}

fn print(log: &Vec<LogType>) {
    for msg in &log[(log.len().saturating_sub(5))..log.len()] {
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
}

pub fn push_print(message: LogType, log: &mut Vec<LogType>) {
    log.push(message);
    cls();
    print(log);
}
