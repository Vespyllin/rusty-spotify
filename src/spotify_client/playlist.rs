use reqwest::Response;

use super::SPOTIFY_PLAYLIST_URL;

pub async fn get_playlist_data(
    id: &str,
    access_token: &str,
    limit: usize,
    offset: usize,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let req_url = format!("{}/{}/tracks", SPOTIFY_PLAYLIST_URL, id);
    let params = [("limit", limit.to_string()), ("offset", offset.to_string())];

    return client
        .get(req_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await;
}
