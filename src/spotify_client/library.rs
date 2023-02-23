use anyhow::Result;
use reqwest::Response;

const SPOTIFY_LIBRARY_URL: &str = "https://api.spotify.com/v1/me";

pub async fn get_liked_tracks(access_token: String, limit: u64, offset: u64) -> Result<Response> {
    let client = reqwest::Client::new();

    let req_url = format!("{}/tracks", SPOTIFY_LIBRARY_URL);
    let params = [("limit", limit.to_string()), ("offset", offset.to_string())];

    return client
        .get(req_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(anyhow::Error::msg);
}

pub async fn get_saved_albums(access_token: String, limit: u64, offset: u64) -> Result<Response> {
    let client = reqwest::Client::new();

    let req_url = format!("{}/albums", SPOTIFY_LIBRARY_URL);
    let params = [("limit", limit.to_string()), ("offset", offset.to_string())];

    return client
        .get(req_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(anyhow::Error::msg);
}

pub async fn get_playlists(access_token: String, limit: u64, offset: u64) -> Result<Response> {
    let client = reqwest::Client::new();

    let req_url = format!("{}/playlists", SPOTIFY_LIBRARY_URL);
    let params = [("limit", limit.to_string()), ("offset", offset.to_string())];

    return client
        .get(req_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(anyhow::Error::msg);
}
