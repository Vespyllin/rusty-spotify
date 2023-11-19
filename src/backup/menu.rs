use crossterm::style::Stylize;

use crate::menu::{cls, handle_input, sep, teardown, Choice, MENU_WIDTH};

pub struct BackupOptions {
    pub liked_tracks: bool,
    pub saved_albums: bool,
    pub saved_playlists: bool,
}

pub fn backup_menu() -> BackupOptions {
    let mut selection = vec![false, false, false];
    backup_menu_core(&mut selection);
    return BackupOptions {
        liked_tracks: selection[0],
        saved_albums: selection[1],
        saved_playlists: selection[2],
    };
}

fn backup_menu_core(selection: &mut Vec<bool>) {
    let options = vec!["Liked tracks.", "Saved albums.", "Saved playlists."];
    cls();
    sep();

    let prompt = "Select which item(s) to export to JSON.";
    let prompt_len: usize = prompt.len();

    let even_space = (0..((MENU_WIDTH - prompt_len) / 2) - 1)
        .map(|_| ' ')
        .collect::<String>();

    println!("{}{}\n", even_space, prompt.bold());

    selection
        .iter()
        .zip(options.iter())
        .enumerate()
        .for_each(|(i, (highlight, option))| {
            let prompt = format!("{}. {}", (i + 1), option);
            if *highlight {
                println!("{}", prompt.black().on_green());
            } else {
                println!("{}", prompt);
            }
        });

    println!();
    println!("q. Quit.");
    println!();
    sep();

    match handle_input() {
        Choice::Exit => teardown(),

        Choice::Option(key) => match key {
            '1' => {
                selection[0] = !selection[0];
                backup_menu_core(selection)
            }
            '2' => {
                selection[1] = !selection[1];
                backup_menu_core(selection)
            }
            '3' => {
                selection[2] = !selection[2];
                backup_menu_core(selection)
            }
            _ => backup_menu_core(selection),
        },
        Choice::Submit => {
            if !selection.iter().fold(false, |acc, &x| acc || x) {
                backup_menu_core(selection)
            }
        }

        _ => backup_menu_core(selection),
    }
}
