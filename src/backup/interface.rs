use anyhow::Result;
use serde_json::Value;

use crate::spotify_client::library::{get_liked_albums, get_liked_tracks, get_playlists};
use crate::spotify_client::playlist::get_playlist_tracks;

use super::parser::{parse_album, parse_playlist_metadata, parse_track};
use super::{ReducedAlbum, ReducedPlaylistMetadata, ReducedTrack};

pub async fn process_liked_tracks(access_token: &str) -> Result<Vec<ReducedTrack>> {
    let mut tracks: Vec<ReducedTrack> = vec![];

    let limit = 50;
    let mut offset = 0;
    loop {
        let res = get_liked_tracks(access_token, limit, offset).await?;
        let body = res.text().await?;
        let parsed_json: Value = serde_json::from_str(&body)?;

        if let Some(items) = parsed_json.get("items").and_then(Value::as_array) {
            for item in items {
                tracks.push(parse_track(item))
            }
        }

        offset += limit;
        let next = parsed_json.get("next").map(|f| f.is_null());
        if next.is_none() || next.unwrap() {
            break;
        }
    }

    Ok(tracks)
}

pub async fn process_liked_albums(access_token: &str) -> Result<Vec<ReducedAlbum>> {
    let mut albums: Vec<ReducedAlbum> = vec![];

    let limit = 50;
    let mut offset = 0;
    loop {
        let res = get_liked_albums(access_token, limit, offset).await?;
        let body = res.text().await?;
        let parsed_json: Value = serde_json::from_str(&body)?;

        if let Some(items) = parsed_json.get("items").and_then(Value::as_array) {
            for item in items {
                albums.push(parse_album(item))
            }
        }

        offset += limit;
        let next = parsed_json.get("next").map(|f| f.is_null());
        if next.is_none() || next.unwrap() {
            break;
        }
    }

    Ok(albums)
}

pub async fn process_playlists(access_token: &str) -> Result<Vec<ReducedPlaylistMetadata>> {
    let mut playlists: Vec<ReducedPlaylistMetadata> = vec![];

    let limit = 50;
    let mut offset = 0;
    loop {
        let res = get_playlists(access_token, limit, offset).await?;
        let body = res.text().await?;
        let parsed_json: Value = serde_json::from_str(&body)?;

        if let Some(items) = parsed_json.get("items").and_then(Value::as_array) {
            for item in items {
                playlists.push(parse_playlist_metadata(item));
            }
        }

        offset += limit;
        let next = parsed_json.get("next").map(|f| f.is_null());
        if next.is_none() || next.unwrap() {
            break;
        }
    }

    Ok(playlists)
}

pub async fn process_playlist_tracks(
    access_token: &str,
    playlist_id: &str,
) -> Result<Vec<ReducedTrack>> {
    let mut tracks: Vec<ReducedTrack> = vec![];

    let limit = 50;
    let mut offset = 0;
    loop {
        let res = get_playlist_tracks(access_token, playlist_id, limit, offset).await?;
        let body = res.text().await?;
        let parsed_json: Value = serde_json::from_str(&body)?;

        if let Some(items) = parsed_json.get("items").and_then(Value::as_array) {
            for item in items {
                tracks.push(parse_track(item))
            }
        }

        offset += limit;
        let next = parsed_json.get("next").map(|f| f.is_null());
        if next.is_none() || next.unwrap() {
            break;
        }
    }

    Ok(tracks)
}
