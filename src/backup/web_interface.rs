use crate::spotify_client::{
    self, AlbumData, ErrorData, GetItemsData, GetItemsResponse,
    GetItemsResponse::{Failure, Success},
    ReducedAlbumData,
};

pub async fn pull_albums_data(access_token: &str) -> Result<Vec<ReducedAlbumData>, String> {
    let mut filtered_data: Vec<ReducedAlbumData> = vec![];

    let limit = 50;
    let mut offset = 0;

    // Loop until the library is fully traversed
    loop {
        // Make a request for the user's albums
        let res = match spotify_client::library::get_saved_albums(access_token, limit, offset).await
        {
            Ok(val) => val,
            Err(err) => {
                let error_msg = format!("Error in Reqwest: {}", err);
                return Err(error_msg);
            }
        };

        // Deserialize the response
        let json_res = match res.json::<GetItemsResponse<AlbumData>>().await {
            Ok(val) => val,
            Err(err) => {
                let error_msg =
                    format!("Error: Could not deserialize get_albums response.\n{}", err);
                return Err(error_msg);
            }
        };

        // Extract data from wrapper
        let response_data: GetItemsData<AlbumData> = match json_res {
            Success(data) => data,
            Failure(data) => {
                let ErrorData { status, message } = data.error;
                let error_msg = format!("Error from Spotify: {} - \"{}\"", status, message);
                return Err(error_msg);
            }
        };

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

    Ok(filtered_data)
}
