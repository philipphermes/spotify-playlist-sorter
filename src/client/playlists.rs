use std::env;
use reqwest::Client;
use colored::Colorize;
use indicatif::ProgressBar;
use crate::model::playlist::{Playlist, PlaylistResponse};

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