pub mod web_interface;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::fs_handler::{get_latest_json, write_data};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReducedAlbum {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub added_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReducedTrack {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub added_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReducedPlaylist {
    pub meta: ReducedPlaylistMetadata,

    pub tracks: Vec<ReducedTrack>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReducedPlaylistMetadata {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_display_name: String,
}

#[derive(Serialize, Debug)]
pub enum ReducedData {
    Track(ReducedTrack),
    Album(ReducedAlbum),
    Playlist(ReducedPlaylist),
}

pub type BackupData = Vec<ReducedData>;

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

pub fn dump_data<T: Serialize>(destination_dir: &str, data: &Vec<T>) -> Result<String, String> {
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

    match write_data(
        destination_dir,
        &format!("{}.json", &new_hash[5..15]),
        &json_data,
    ) {
        Ok(_) => return Ok("Successfully backed up data".to_owned()),
        Err(e) => {
            return Err(format!(
                "Failed to backup data to {}.\n{}",
                destination_dir, e
            ))
        }
    }
}
