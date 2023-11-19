use anyhow::{bail, Result};
use reqwest::Response;

use super::get;

const SPOTIFY_LIBRARY_URL: &str = "https://api.spotify.com/v1/me";

pub async fn get_liked_tracks(access_token: &str, limit: u64, offset: u64) -> Result<Response> {
    let req_url = format!("{}/tracks", SPOTIFY_LIBRARY_URL);
    let params = [
        ("limit".to_string(), limit.to_string()),
        ("offset".to_string(), offset.to_string()),
    ];

    let request = get(access_token, req_url, &params).send().await?;

    if request.status().as_u16() != 200 {
        bail!("Request failed with status code: {}", request.status());
    }

    Ok(request)
}

pub async fn get_liked_albums(access_token: &str, limit: u64, offset: u64) -> Result<Response> {
    let req_url = format!("{}/albums", SPOTIFY_LIBRARY_URL);
    let params = [
        ("limit".to_string(), limit.to_string()),
        ("offset".to_string(), offset.to_string()),
    ];

    let request = get(access_token, req_url, &params).send().await?;

    if request.status().as_u16() != 200 {
        bail!("Request failed with status code: {}", request.status());
    }

    Ok(request)
}

pub async fn get_playlists(access_token: &str, limit: u64, offset: u64) -> Result<Response> {
    let req_url = format!("{}/playlists", SPOTIFY_LIBRARY_URL);
    let params = [
        ("limit".to_string(), limit.to_string()),
        ("offset".to_string(), offset.to_string()),
    ];

    let request = get(access_token, req_url, &params).send().await?;

    if request.status().as_u16() != 200 {
        bail!("Request failed with status code: {}", request.status());
    }

    Ok(request)
}
