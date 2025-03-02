use dotenv::dotenv;
use client::auth;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::client::playlists::get_playlist;
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
    let playlists_resp = get_playlist(global_token.to_string()).await;

    match playlists_resp {
        Ok(playlists) => {println!("{:?}", playlists)}
        Err(err) => {println!("{:?}", err.to_string())}
    }
}