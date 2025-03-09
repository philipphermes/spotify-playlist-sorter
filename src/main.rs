use crate::client::auth;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::Mutex;
use colored::Colorize;
use crate::client::artist::get_artists;
use crate::client::playlists::get_playlists;
use crate::client::saved_tracks::get_saved_tracks;
use crate::model::playlist::{Owner, Playlist};
use crate::utils::sort::sort;

mod client;
mod model;
mod utils;

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

    let mut playlists = match get_playlists(global_token.to_string()).await {
        Ok(playlists) => playlists,
        Err(e) => {
            eprintln!("Error fetching playlists: {:?}", e); // Log the error
            return;
        }
    };

    let saved_tracks = match get_saved_tracks(global_token.to_string()).await {
        Ok(saved_tracks) => saved_tracks,
        Err(e) => {
            eprintln!("Error fetching saved tracks: {:?}", e); // Log the error
            return;
        }
    };


    let mut artist_ids: Vec<String> = Vec::new();

    for saved_track in saved_tracks.clone() {
        for artist in saved_track.track.artists {
            let artist_id = artist.id;

            if !artist_ids.contains(&artist_id) {
                artist_ids.push(artist_id);
            }
        }
    }

    let artists = get_artists(global_token.to_string(), artist_ids).await;

    let artists = match artists {
        Ok(artist) => artist,
        Err(_) => return,
    };

    let new_playlist = Playlist {
        id: String::from("Miscellaneous"),
        name: String::from("Miscellaneous"),
        owner: Owner {
            id: String::from("Miscellaneous")
        },
        songs: None,
    };

    playlists.push(new_playlist);

    sort(saved_tracks, &mut playlists, artists);

    for playlist in playlists {
        println!("{}:", playlist.name.blue());

        if let Some(songs) = playlist.songs {
            for track in songs {
                println!("\t- {}", track.name.green());
            }
        } else {
            println!("\t{}", "No songs available in this playlist.".red());
        }
    }
}
