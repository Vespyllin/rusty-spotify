mod backup;
mod main_menu;
mod menu;
mod spotify_client;

use backup::{backup_liked_albums, backup_liked_tracks, backup_playlists, menu::backup_menu};

#[tokio::main]
async fn main() {
    let access_token = "";
    let destination_dir = "/mnt/c/Users/mystech/Desktop/"; // TODO: Make env var or './'
    menu::setup();

    let menu_option = main_menu::menu();
    if menu_option == main_menu::MenuOptions::Backup {
        let backup_options = backup_menu();

        if backup_options.liked_tracks {
            backup_liked_tracks(access_token, &destination_dir)
                .await
                .unwrap();
        }
        if backup_options.saved_albums {
            backup_liked_albums(access_token, &destination_dir)
                .await
                .unwrap();
        }
        if backup_options.saved_playlists {
            backup_playlists(access_token, &destination_dir)
                .await
                .unwrap();
        }
    } else {
        todo!("Restore menu");
    };

    menu::teardown();
}
