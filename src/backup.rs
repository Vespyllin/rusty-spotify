use crate::spotify_client::{
    self, AlbumData, GetItemsData, GetItemsResponse,
    GetItemsResponse::{Failure, Success},
    ReducedAlbumData,
};

pub async fn pull_albums_data(access_token: &str) -> Result<String, String> {
    let mut filtered_data: Vec<ReducedAlbumData> = vec![];

    let limit = 50;
    let mut offset = 0;

    // Loop until the library is fully traversed
    loop {
        // Make a request for the user's albums
        let raw_res = spotify_client::library::get_saved_albums(access_token, limit, offset).await;

        if raw_res.is_err() {
            let error_msg = format!("Error in Rekwest: {}", raw_res.unwrap_err());
            return Err(error_msg);
        }

        // Deserialize the response
        let json_res = raw_res.unwrap().json::<GetItemsResponse<AlbumData>>().await;
        if json_res.is_err() {
            let error_msg = format!(
                "Error: Could not deserialize get_albums response.\n{}",
                json_res.unwrap_err()
            );
            return Err(error_msg);
        }

        // Extract data from wrapper
        let res = json_res.unwrap();
        let response_data: GetItemsData<AlbumData>;

        match res {
            Success(data) => response_data = data,
            Failure(data) => {
                let error_msg = format!(
                    "Error from Spotify: {} - \"{}\"",
                    data.error.status, data.error.message
                );
                return Err(error_msg);
            }
        }

        // Filter the album's fields
        let mut filtered_album: Vec<ReducedAlbumData> = response_data
            .items
            .into_iter()
            .map(|item| ReducedAlbumData {
                id: item.album.id,
                name: item.album.name,
                artist: item.album.artists[0].name.to_owned(),
                added_at: item.added_at.unwrap_or("never".to_string()),
            })
            .collect();

        // Collect the reduced data into a vector
        filtered_data.append(&mut filtered_album);
        offset += limit;

        // Break when the API signals that we have traversed all albums
        if response_data.next.is_none() {
            break;
        }
    }

    let reduced_json = serde_json::to_string_pretty(&filtered_data);
    if reduced_json.is_err() {
        let error_msg = format!(
            "Error: Could not serialize a reduced response.\n{}",
            reduced_json.unwrap_err()
        );

        return Err(error_msg);
    }

    return Ok(reduced_json.unwrap());
}
