pub mod web_interface;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    fs_handler::{get_latest_json, write_data},
    spotify_client::FinalState,
};

fn get_hash(data: &str) -> String {
    format!(
        "{:x}",
        Sha256::digest(
            data.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .as_bytes()
        )
    )
}

pub fn backup_handler<T: FinalState + Serialize>(
    destination_dir: &str,
    data: &Vec<T>,
) -> Result<String, String> {
    let json_data = match serde_json::to_string_pretty(&data) {
        Ok(val) => val,
        Err(_) => return Err("Could not serialize backup data".to_string()),
    };

    let new_hash = get_hash(&json_data);

    let existing_backup = match get_latest_json(destination_dir) {
        Ok(val) => val,
        Err(_) => None,
    };

    if existing_backup.is_some() && get_hash(&existing_backup.unwrap()) == new_hash {
        return Ok("Existing backup is up to date.".to_owned());
    }

    match write_data(destination_dir, &new_hash[5..15], &json_data) {
        Ok(_) => return Ok("Successfully backed up data".to_owned()),
        Err(_) => return Err(format!("Failed to backup data to {}", destination_dir)),
    }
}
