use reqwest::RequestBuilder;

pub mod library;
pub mod playlist;

fn get(access_token: &str, url: String, params: &[(String, String)]) -> RequestBuilder {
    reqwest::Client::new()
        .get(url)
        .query(params)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
}
