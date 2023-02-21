use std::future::Future;

use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::spotify_client::{
    self,
    library::{get_liked_tracks, get_saved_albums},
    Album, AlbumData, ErrorData, LikedTrackData, PlaylistMetadata, PlaylistTracksData, SpotifyData,
    SpotifyResponse,
    SpotifyResponse::{Failure, Success},
    Track,
};

use super::{BackupData, ReducedAlbum, ReducedData, ReducedPlaylistMetadata, ReducedTrack};

async fn pull_parse<T, Fut, R: DeserializeOwned>(
    access_token: String,
    limit: u64,
    offset: u64,
    func: T,
) -> Result<SpotifyData<R>, String>
where
    T: Fn(String, u64, u64) -> Fut,
    Fut: Future<Output = Result<Response, reqwest::Error>>,
{
    let res = match func(access_token, limit, offset).await {
        Ok(val) => val,
        Err(err) => {
            let error_msg = format!("Error in Reqwest: {}", err);
            return Err(error_msg);
        }
    };

    // Deserialize the response
    let json_res: SpotifyResponse<R> = match res.json().await {
        Ok(val) => val,
        Err(err) => {
            let error_msg = format!("Error: Could not deserialize get_likes response.\n{}", err);
            return Err(error_msg);
        }
    };

    // Extract data from wrapper
    // let response_data: SpotifyData<R> =
    return match json_res {
        Success(data) => Ok(data),
        Failure(data) => {
            let ErrorData { status, message } = data.error;
            let error_msg = format!("Error from Spotify: {} - \"{}\"", status, message);
            return Err(error_msg);
        }
    };
}

pub async fn pull_liked_tracks(access_token: &str) -> Result<BackupData, String> {
    let mut filtered_data: BackupData = vec![];

    let limit = 50;
    let mut offset = 0;

    // Loop until the library is fully traversed
    loop {
        let res = pull_parse(access_token.to_owned(), limit, offset, get_liked_tracks).await?;

        // Filter the tracks' fields
        let mut filtered_tracks: BackupData = res
            .items
            .into_iter()
            .map(|item: LikedTrackData| {
                let Track { id, name, artists } = item.track;
                return ReducedData::Track(ReducedTrack {
                    added_at: Some(item.added_at),
                    id: id.or(Some("local".to_owned())),
                    name: name,
                    artist: artists[0].name.to_owned(),
                });
            })
            .collect();

        // Collect the reduced data into a vector
        filtered_data.append(&mut filtered_tracks);
        offset += limit;

        // Break when the API signals that we have traversed all liked tracks
        if res.next.is_none() {
            break;
        }
    }

    Ok(filtered_data)
}

pub async fn pull_albums(access_token: &str) -> Result<BackupData, String> {
    let mut filtered_data: BackupData = vec![];

    let limit = 50;
    let mut offset = 0;

    // Loop until the library is fully traversed
    loop {
        let res = pull_parse(access_token.to_owned(), limit, offset, get_saved_albums).await?;

        // Filter the album's fields
        let mut filtered_album: BackupData = res
            .items
            .into_iter()
            .map(|item: AlbumData| {
                let Album {
                    id, name, artists, ..
                } = item.album;

                ReducedData::Album(ReducedAlbum {
                    id,
                    name,
                    artist: artists[0].name.to_owned().unwrap_or("local".to_owned()),
                    added_at: item.added_at,
                })
            })
            .collect();

        // Collect the reduced data into a vector
        filtered_data.append(&mut filtered_album);
        offset += limit;

        // Break when the API signals that we have traversed all albums
        if res.next.is_none() {
            break;
        }
    }

    Ok(filtered_data)
}

pub async fn pull_playlists(access_token: &str) -> Result<Vec<ReducedPlaylistMetadata>, String> {
    let mut filtered_data: Vec<ReducedPlaylistMetadata> = vec![];

    let limit = 50;
    let mut offset = 0;

    // Loop until the library is fully traversed
    loop {
        let res = pull_parse(access_token.to_owned(), limit, offset, get_saved_albums).await?;

        // Filter the tracks' fields
        let mut filtered_playlists: Vec<ReducedPlaylistMetadata> = res
            .items
            .into_iter()
            .map(|item| {
                let PlaylistMetadata {
                    id,
                    name,
                    description,
                    owner,
                    ..
                } = item;

                ReducedPlaylistMetadata {
                    id,
                    name,
                    description,
                    owner_display_name: owner.display_name.unwrap_or("Spotify".to_owned()),
                }
            })
            .collect();

        // Collect the reduced data into a vector
        filtered_data.append(&mut filtered_playlists);
        offset += limit;

        // Break when the API signals that we have traversed all liked tracks
        if res.next.is_none() {
            break;
        }
    }

    Ok(filtered_data)
}

pub async fn pull_playlist_tracks(
    access_token: &str,
    playlist_id: &str,
) -> Result<Vec<ReducedTrack>, String> {
    let mut filtered_data: Vec<ReducedTrack> = vec![];

    let limit = 50;
    let mut offset = 0;

    // Loop until the library is fully traversed
    loop {
        // Make a request for the user's albums
        let res = match spotify_client::playlist::get_playlist_data(
            playlist_id,
            access_token,
            limit,
            offset,
        )
        .await
        {
            Ok(val) => val,
            Err(err) => {
                let error_msg = format!("Error in Reqwest: {}", err);
                return Err(error_msg);
            }
        };

        // Deserialize the response
        let json_res: SpotifyResponse<PlaylistTracksData> = match res.json().await {
            Ok(val) => val,
            Err(err) => {
                let error_msg = format!(
                    "Error: Could not deserialize get_playlists response.\n{}",
                    err
                );
                return Err(error_msg);
            }
        };
        // Extract data from wrapper
        let response_data: SpotifyData<PlaylistTracksData> = match json_res {
            Success(data) => data,
            Failure(data) => {
                let ErrorData { status, message } = data.error;
                let error_msg = format!("Error from Spotify: {} - \"{}\"", status, message);
                return Err(error_msg);
            }
        };

        let mut filtered_playlists: Vec<ReducedTrack> = response_data
            .items
            .into_iter()
            .map(|item| {
                let PlaylistTracksData {
                    added_at, track, ..
                } = item;

                ReducedTrack {
                    id: track.id,
                    name: track.name,
                    artist: track.artists[0].name.to_owned(),
                    added_at: Some(added_at),
                }
            })
            .collect();

        filtered_data.append(&mut filtered_playlists);
        offset += limit;

        // Break when the API signals that we have traversed all liked tracks
        if response_data.next.is_none() {
            break;
        }
    }

    Ok(filtered_data)
}
