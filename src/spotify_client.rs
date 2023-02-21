use serde::Deserialize;

const SPOTIFY_LIBRARY_URL: &str = "https://api.spotify.com/v1/me";
const SPOTIFY_PLAYLIST_URL: &str = "https://api.spotify.com/v1/playlists";

pub mod library;
pub mod playlist;

#[derive(Deserialize, Debug)]
pub struct Image {
    pub url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub id: Option<String>,
    pub display_name: Option<String>,

    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistTracksData {
    pub added_at: String,
    pub added_by: Owner,
    pub track: Track,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistMetadata {
    pub collaborative: Option<bool>,

    pub id: String,
    pub name: String,
    pub description: Option<String>,

    pub images: Vec<Image>,
    pub owner: Owner,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub genres: Option<Vec<String>>,

    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumTrack {
    pub id: Option<String>,
    pub name: Option<String>,
    pub track_number: Option<u64>,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumTrackData {
    pub items: Vec<AlbumTrack>,

    pub limit: u64,
    pub offset: u64,
    pub total: u64,

    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub album_type: String,

    pub images: Vec<Image>,

    #[serde(rename = "type")]
    pub _type: String,

    pub total_tracks: u64,
    pub tracks: AlbumTrackData,

    pub genres: Vec<String>,
    pub label: String,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumData {
    pub added_at: String,
    pub album: Album,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: Option<String>,
    pub name: Option<String>,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
pub struct LikedTrackData {
    pub added_at: String,
    pub track: Track,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyData<T> {
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
pub enum SpotifyResponse<T> {
    Success(SpotifyData<T>),
    Failure(Error),
}
