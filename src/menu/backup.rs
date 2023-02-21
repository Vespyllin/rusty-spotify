use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::menu::{sep, WIDTH};

use super::{reset_cursor, Choice};

fn print_menu(selected: &mut Vec<bool>) {
    let options = vec!["Liked tracks.", "Saved albums.", "Saved playlists."];

    assert_eq!(
        selected.len(),
        options.len(),
        "Selection and options vectors are not of the same length."
    );

    sep();

    let prompt = "Select which option/s to export to JSON.";
    let prompt_len: usize = prompt.len() - 9;

    let even_space = (0..((WIDTH - prompt_len) / 2) - 1)
        .map(|_| ' ')
        .collect::<String>();

    println!("{}{}\n", even_space, prompt.bold());

    selected
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

    println!("q. Quit.");

    sep();
}

fn handle_input() -> Choice {
    let choice: Choice;

    enable_raw_mode().unwrap();

    match read().unwrap() {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => choice = Choice::Exit,

        Event::Key(KeyEvent {
            code: KeyCode::Char(x),
            ..
        }) => match x {
            '1' => choice = Choice::Option(1),

            '2' => choice = Choice::Option(2),

            '3' => choice = Choice::Option(3),

            'q' => choice = Choice::Exit,

            _ => choice = Choice::Default,
        },

        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => choice = Choice::Submit,

        _ => choice = Choice::Default,
    }

    // Double input workaround for Windows
    if cfg!(windows) {
        _ = read().unwrap();
    }

    disable_raw_mode().unwrap();

    choice
}

pub fn menu() -> Option<Vec<bool>> {
    let mut selection: Vec<bool> = vec![false, false, false];

    loop {
        print_menu(&mut selection);

        match handle_input() {
            Choice::Submit => {
                // If at least 1 option is selected, allow the user to continue
                if selection.iter().fold(false, |acc, v| acc || *v) {
                    break;
                }
            }
            Choice::Option(x) => selection[x - 1] = !selection[x - 1],
            Choice::Exit => return None,
            _ => {}
        }

        reset_cursor();
    }

    Some(selection)
}
