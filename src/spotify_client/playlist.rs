use anyhow::Result;
use reqwest::Response;

use super::get;

const SPOTIFY_PLAYLIST_URL: &str = "https://api.spotify.com/v1/playlists";

pub async fn get_playlist_tracks(
    access_token: &str,
    id: &str,
    limit: usize,
    offset: usize,
) -> Result<Response> {
    let req_url = format!("{}/{}/tracks", SPOTIFY_PLAYLIST_URL, id);
    let params = [
        ("limit".to_string(), limit.to_string()),
        ("offset".to_string(), offset.to_string()),
    ];

    let request = get(access_token, req_url, &params)
        .send()
        .await
        .map_err(anyhow::Error::msg);

    if let Ok(data) = &request {
        if data.status().as_u16() != 200 {
            return Err(anyhow::Error::msg(format!(
                "Request failed with status code: {}",
                data.status()
            )));
        }
    }

    request
}
