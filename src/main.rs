use crate::client::auth;
use crate::client::playlists::get_playlist;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::client::artist::get_artists;
use crate::client::saved_tracks::get_saved_tracks;

mod client;
mod model;

lazy_static! {
    static ref TOKEN: Mutex<String> = Mutex::new(String::from("initial_token_value"));
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    auth::auth().await;

    let global_token = TOKEN.lock().unwrap();
    let playlists_resp = get_saved_tracks(global_token.to_string()).await;
    let artists_resp = get_artists(global_token.to_string(), playlists_resp.unwrap().items[0].track.artists.clone()).await;

    match artists_resp {
        Ok(playlists) => {
            println!("{:?}", playlists)
        }
        Err(err) => {
            println!("{:?}", err.to_string())
        }
    }
}
