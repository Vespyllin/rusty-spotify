#![allow(unused_variables)]
#![allow(unused_imports)]

use std::future::Future;

use reqwest::{Error, RequestBuilder, Response};

use super::SPOTIFY_LIBRARY_URL;

pub async fn get_liked_tracks(
    access_token: &str,
    limit: usize,
    offset: usize,
) -> Result<Response, reqwest::Error> {
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
        .await;
}

pub async fn get_saved_albums(
    access_token: &str,
    limit: usize,
    offset: usize,
) -> Result<Response, reqwest::Error> {
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
        .await;
}

pub async fn get_playlists(
    access_token: &str,
    limit: usize,
    offset: usize,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let req_url = format!("{}/playlists", SPOTIFY_LIBRARY_URL);
    let params = [("limit", limit.to_string()), ("offset", offset.to_string())];

    // println!("{:#?}", req.try_clone().unwrap().build().unwrap().url());

    return client
        .get(req_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await;
}
