use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use self::{fs::write_data, interface::process_playlist_tracks};

mod fs;
mod interface;
pub mod menu;
mod parser;

#[derive(Serialize, Deserialize, Debug)]
struct ReducedAlbum {
    pub id: Option<String>,
    pub name: Option<String>,
    pub artist: Option<String>,
    pub added_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReducedTrack {
    pub id: Option<String>,
    pub name: Option<String>,
    pub artist: Option<String>,
    pub added_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReducedPlaylist {
    pub meta: ReducedPlaylistMetadata,
    pub tracks: Vec<ReducedTrack>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReducedPlaylistMetadata {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner_display_name: Option<String>,
}

fn time_hash() -> String {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    format!("{:x}", current_time.as_secs())
}

pub async fn backup_liked_tracks(access_token: &str, destination_dir: &str) -> Result<()> {
    let track_backup = interface::process_liked_tracks(access_token).await?;
    write_data(
        &track_backup,
        destination_dir,
        format!("liked_tracks_{}.json", time_hash()).as_str(),
    )?;
    return Ok(());
}

pub async fn backup_liked_albums(access_token: &str, destination_dir: &str) -> Result<()> {
    let album_backup = interface::process_liked_albums(access_token).await?;
    write_data(
        &album_backup,
        destination_dir,
        format!("liked_albums_{}.json", time_hash()).as_str(),
    )?;
    return Ok(());
}

pub async fn backup_playlists(access_token: &str, destination_dir: &str) -> Result<()> {
    let playlist_metadata = interface::process_playlists(access_token).await?;
    let mut paylists_backup: Vec<ReducedPlaylist> = vec![];

    for metadata in playlist_metadata {
        if let Some(id) = metadata.id.clone() {
            paylists_backup.push(ReducedPlaylist {
                meta: metadata,
                tracks: process_playlist_tracks(access_token, &id).await?,
            });
        } else {
            bail!("Could not retrieve playlist tracks");
        }
    }

    write_data(
        &paylists_backup,
        destination_dir,
        format!("playlists_{}.json", time_hash()).as_str(),
    )?;

    return Ok(());
}
