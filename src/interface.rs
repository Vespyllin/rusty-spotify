pub mod fs;
pub mod spotify;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReducedAlbum {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub added_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReducedTrack {
    pub id: Option<String>,
    pub name: Option<String>,
    pub artist: Option<String>,
    pub added_at: Option<String>,
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
#[serde(untagged)]
pub enum ReducedData {
    Track(ReducedTrack),
    Album(ReducedAlbum),
    Playlists(ReducedPlaylist),
}

pub type BackupData = Vec<ReducedData>;
