use serde::{Deserialize, Serialize};

const SPOTIFY_LIBRARY_URL: &str = "https://api.spotify.com/v1/me";

pub mod library;

#[derive(Deserialize, Debug)]
pub struct PlaylistTrackMetaData {
    pub href: Option<String>,
    pub total: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ExternalUrl {
    pub spotify: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistData {
    pub collaborative: Option<bool>,
    pub description: Option<String>,

    pub external_urls: Option<ExternalUrl>,
    pub href: Option<String>,

    pub id: Option<String>,
    pub images: Option<Vec<Image>>,
    pub name: Option<String>,
    // pub owner: Owner,
    pub public: Option<bool>,
    pub snapshot_id: Option<String>,
    pub tracks: Option<PlaylistTrackMetaData>,

    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub uri: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Artist {
    pub external_urls: Option<ExternalUrl>,
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub artists: Vec<Artist>,
    pub duration_ms: Option<u64>,
    pub explicit: Option<bool>,

    pub href: Option<String>,
    pub id: Option<String>,
    pub is_playable: Option<bool>,

    pub name: Option<String>,
    pub track_number: Option<u64>,

    #[serde(rename = "type")]
    pub _type: Option<String>,

    pub uri: Option<String>,
    pub is_local: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct TrackData {
    pub href: String,
    pub items: Vec<Track>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,

    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub album_type: String,
    pub total_tracks: u64,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_data_precision: Option<String>,

    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,

    pub genres: Vec<String>,
    pub label: String,
    pub artists: Vec<Artist>,
    pub tracks: TrackData,
}

#[derive(Deserialize, Debug)]
pub struct AlbumData {
    pub added_at: Option<String>,
    pub album: Album,
}

#[derive(Deserialize, Debug)]
pub struct GetItemsData<T> {
    pub href: String,

    pub items: Vec<T>,

    pub limit: u64,
    pub offset: u64,
    pub total: u64,

    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub error: ErrorData,
}
#[derive(Deserialize, Debug)]
pub struct ErrorData {
    pub status: u64,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetItemsResponse<T> {
    Success(GetItemsData<T>),
    Failure(Error),
}

#[derive(Serialize, Deserialize)]
pub struct ReducedAlbumData {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub added_at: String,
}
