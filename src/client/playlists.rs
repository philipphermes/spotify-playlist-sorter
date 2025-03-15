use std::env;
use reqwest::Client;
use colored::Colorize;
use indicatif::ProgressBar;
use serde_json::json;
use crate::model::playlist::{Playlist, PlaylistAddResponse, PlaylistCreateResponse, PlaylistResponse};
use crate::model::saved_tracks::Track;

pub async fn get_playlists(token: String) -> Result<Vec<Playlist>, reqwest::Error> {
    println!("\n\n{}", "Fetching playlists".green().bold());

    let mut playlists: Vec<Playlist> = Vec::new();
    let mut next_playlists: Option<String> = None;

    let mut playlist_pb = None;
    let mut fetched_playlists: u64 = 0;
    let mut total_playlists: u64 = 0;

    loop {
        let playlists_resp = fetch_playlists(token.clone(), next_playlists.clone()).await;

        let playlists_response = match playlists_resp {
            Ok(playlists) => playlists,
            Err(err) => return Err(err),
        };

        playlists.append(&mut playlists_response.clone().items);
        fetched_playlists += playlists_response.items.len() as u64;
        next_playlists = playlists_response.next;

        if total_playlists == 0 {
            total_playlists = playlists_response.total;
            playlist_pb = Some(ProgressBar::new(total_playlists));
        }

        if let Some(ref pb) = playlist_pb {
            pb.set_position(fetched_playlists);
        }

        if next_playlists.is_none() {
            if let Some(ref pb) = playlist_pb {
                pb.finish_with_message("done");
            }
            break;
        }
    }

    let user_id = env::var("USER_ID").expect("USER_ID not set");

    let mut owned_playlists: Vec<Playlist> = Vec::new();

    for playlist in playlists {
        if playlist.owner.id == user_id {
            owned_playlists.push(playlist);
        }
    }

    Ok(owned_playlists)
}

pub async fn add_to_playlist(token: String, mut playlist: Playlist) -> Result<PlaylistAddResponse, reqwest::Error> {
    if playlist.id == playlist.name.clone() {
        let playlists_create_resp = create_playlist(token.clone(), playlist.clone()).await;

        let playlists_create_response = match playlists_create_resp {
            Ok(playlists) => playlists,
            Err(err) => return Err(err),
        };

        playlist.id = playlists_create_response.id;
    }

    let mut songs_ids: Vec<String> = Vec::new();
    let mut playlists_add_response = PlaylistAddResponse { snapshot_id: "".to_string() };

    if let Some(mut playlist_songs) = playlist.clone().songs {
        for mut song in playlist_songs.clone() {
            songs_ids.push(format!("spotify:track:{}", song.id));

            if songs_ids.clone().len() == 100 || songs_ids.clone().len() == playlist_songs.clone().len() {
                println!("added {} songs to playlist {} :{:#?}", songs_ids.len(), playlist.name, songs_ids);

                let playlists_add_resp = playlist_add(token.clone(), playlist.clone().id, songs_ids.clone()).await;

                playlists_add_response = match playlists_add_resp {
                    Ok(playlists) => playlists,
                    Err(err) => return Err(err),
                };

                songs_ids = Vec::new();
            }
        }
    }

    Ok(playlists_add_response)
}

async fn create_playlist(token: String, playlist: Playlist) -> Result<PlaylistCreateResponse, reqwest::Error> {
    let user_id = env::var("USER_ID").expect("USER_ID not set");
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let url = format!("https://api.spotify.com/v1/users/{}/playlists", user_id);
    let playlist_response = client
        .post(url)
        .header("Authorization", auth_header_value)
        .json(&json!({
                "name": playlist.name.clone(),
                "public": false,
                "collaborative": false,
                "description": "auto generated playlist by spotify sorter"
            }))
        .send()
        .await?;

    let playlist_response_body: PlaylistCreateResponse = playlist_response.json().await?;

    Ok(playlist_response_body)
}

async fn playlist_add(token: String, playlist_id: String, songs: Vec<String>) -> Result<PlaylistAddResponse, reqwest::Error> {
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id);

    let playlist_add_response = client
        .post(url)
        .header("Authorization", auth_header_value)
        .json(&json!({
            "uris": songs,
            "position": 0
            }))
        .send()
        .await?;

    let playlist_add_response_body: PlaylistAddResponse = playlist_add_response.json().await?;

    Ok(playlist_add_response_body)
}

async fn fetch_playlists(
    token: String,
    next: Option<String>,
) -> Result<PlaylistResponse, reqwest::Error> {
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let url = next.unwrap_or_else(|| String::from("https://api.spotify.com/v1/me/playlists?limit=20"));
    let request_url = url.as_str();

    let response = client
        .get(request_url)
        .header("Authorization", auth_header_value)
        .send()
        .await?;

    let response_body: PlaylistResponse = response.json().await?;

    Ok(response_body)
}