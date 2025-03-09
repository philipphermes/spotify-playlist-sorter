use crate::client::auth;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::Mutex;
use colored::Colorize;
use reqwest::Error;
use crate::client::artist::get_artists;
use crate::client::playlists::get_playlists;
use crate::client::saved_tracks::get_saved_tracks;
use crate::model::saved_tracks::SavedTrack;

mod client;
mod model;

lazy_static! {
    static ref TOKEN: Mutex<String> = Mutex::new(String::from("initial_token_value"));
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(" _____             _   _  __        ______ _             _ _     _     _____            _            ");
    println!("/  ___|           | | (_)/ _|       | ___ \\ |           | (_)   | |   /  ___|          | |           ");
    println!("\\ `--. _ __   ___ | |_ _| |_ _   _  | |_/ / | __ _ _   _| |_ ___| |_  \\ `--.  ___  _ __| |_ ___ _ __ ");
    println!(" `--. \\ '_ \\ / _ \\| __| |  _| | | | |  __/| |/ _` | | | | | / __| __|  `--. \\/ _ \\| '__| __/ _ \\ '__|");
    println!("/\\__/ / |_) | (_) | |_| | | | |_| | | |   | | (_| | |_| | | \\__ \\ |_  /\\__/ / (_) | |  | ||  __/ |   ");
    println!("\\____/| .__/ \\___/ \\__|_|_|  \\__, | \\_|   |_|\\__,_|\\__, |_|_|___/\\__| \\____/ \\___/|_|   \\__\\___|_|   ");
    println!("      | |                     __/ |                 __/ |                                            ");
    println!("      |_|                    |___/                 |___/                                             ");

    auth::auth().await;

    let global_token = TOKEN.lock().unwrap();

    let playlists = get_playlists(global_token.to_string()).await;

    let _playlists = match playlists {
        Ok(playlists) => playlists,
        Err(_) => return,
    };

    let saved_tracks = get_saved_tracks(global_token.to_string()).await;

    let saved_tracks = match saved_tracks {
        Ok(saved_track) => saved_track,
        Err(_) => return,
    };

    let mut artist_ids: Vec<String> = Vec::new();

    for saved_track in saved_tracks {
        for artist in saved_track.track.artists {
            let artist_id = artist.id;

            if !artist_ids.contains(&artist_id) {
                artist_ids.push(artist_id);
            }
        }
    }

    let _artists_resp = get_artists(global_token.to_string(), artist_ids).await;

    /*
    match artists_resp {
        Ok(artists) => println!("{:#?}", artists),
        Err(_) => return,
    };

     */
}
