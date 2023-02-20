use backup::{backup_handler, web_interface::pull_albums_data};
use crossterm::event::read;
use std::{process::exit, thread, time::Duration};

use menu::{cls, hide_cursor, reset_cursor, reset_terminal, Choice};

pub mod backup;
mod fs_handler;
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
        _ = read().expect("failed to read input. main");
    }

    let backup_mode: Mode = main_menu();
    cls();

    let access_token = "BQDP_chcndqQHDKm_74ak7MpwzB2dUIMWdZZU77Ola3ZrYLGR8BmSswe6p8d0w4lXWXTuqcrYhk5coqfYQTGszaa7dVMkZIQ_XhxrHtn1ow7JslvSr1Bbq0fpSG73VmMCFSr_d__j1QLkugi8g9-S-BrjmxuoWokvNn6o5ubx_Vv4YYv3gjS7ughJmgBDrxuXBlnng";
    match backup_mode {
        Mode::Backup => {
            let selection = backup_menu();
            cls();

            // if selection[0] {}

            // if selection[1] {}

            if selection[2] {
                let x = pull_albums_data(access_token).await;

                if x.is_ok() {
                    let y = x.unwrap();
                    match backup_handler("./albums/", &y) {
                        Ok(val) => println!("{}", val),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    thread::sleep(Duration::from_secs(3));
    reset_terminal();
}
