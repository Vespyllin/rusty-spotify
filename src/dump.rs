use crate::spotify_client::{AlbumData, GetItemsData};

pub fn dump_albums(album_res: GetItemsData<AlbumData>) {
    for (i, item) in album_res.items.iter().enumerate() {
        let x = &item.album.artists[0].name;

        println!(
            "{}. {} - {} | {}",
            i,
            item.album.name,
            x,
            item.added_at.to_owned().unwrap()
        );
    }
}
