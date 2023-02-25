use anyhow::Result;
use reqwest::Response;

const SPOTIFY_PLAYLIST_URL: &str = "https://api.spotify.com/v1/playlists";

pub async fn get_playlist_data(id: &str, access_token: &str, limit: usize, offset: usize) -> Result<Response> {
    let client = reqwest::Client::new();

    let req_url = format!("{}/{}/tracks", SPOTIFY_PLAYLIST_URL, id);
    let params = [("limit", limit.to_string()), ("offset", offset.to_string())];

    client
        .get(req_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(anyhow::Error::msg)
}
