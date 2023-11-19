use serde_json::Value;

use super::{ReducedAlbum, ReducedPlaylistMetadata, ReducedTrack};

pub fn parse_track(item: &Value) -> ReducedTrack {
    let added_at = item
        .get("added_at")
        .map(|f| f.to_string().replace("\"", ""));

    let id = if let Some(track) = item.get("track") {
        track.get("id").map(|f| f.to_string().replace("\"", ""))
    } else {
        None
    };

    let name = if let Some(track) = item.get("track") {
        track.get("name").map(|f| f.to_string().replace("\"", ""))
    } else {
        None
    };

    let artist = item.get("track").and_then(|track| {
        track.get("artists").and_then(|artists| {
            artists.get(0).and_then(|artist| {
                artist
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
            })
        })
    });

    ReducedTrack {
        id,
        name,
        artist,
        added_at,
    }
}

pub fn parse_album(item: &Value) -> ReducedAlbum {
    let added_at = item
        .get("added_at")
        .map(|f| f.to_string().replace("\"", ""));

    let id = if let Some(album) = item.get("album") {
        album.get("id").map(|f| f.to_string().replace("\"", ""))
    } else {
        None
    };

    let name = if let Some(album) = item.get("album") {
        album.get("name").map(|f| f.to_string().replace("\"", ""))
    } else {
        None
    };

    let artist = item.get("album").and_then(|album| {
        album.get("artists").and_then(|artists| {
            artists.get(0).and_then(|artist| {
                artist
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
            })
        })
    });

    ReducedAlbum {
        id,
        name,
        artist,
        added_at,
    }
}

pub fn parse_playlist_metadata(item: &Value) -> ReducedPlaylistMetadata {
    let id = item.get("id").map(|f| f.to_string().replace("\"", ""));

    let name = item.get("name").map(|f| f.to_string().replace("\"", ""));

    let description = item
        .get("description")
        .map(|f| f.to_string().replace("\"", ""));

    let owner_display_name = item
        .get("owner")
        .and_then(|owner| owner.get("display_name"))
        .map(|f| f.to_string().replace("\"", ""));

    ReducedPlaylistMetadata {
        id,
        name,
        description,
        owner_display_name,
    }
}
