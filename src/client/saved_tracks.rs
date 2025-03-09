use crate::model::saved_tracks::{SavedTrack, SavedTrackResponse};
use colored::Colorize;
use indicatif::ProgressBar;
use reqwest::Client;

pub async fn get_saved_tracks(token: String) -> Result<Vec<SavedTrack>, reqwest::Error> {
    println!("\n\n{}", "Fetching liked songs".green().bold());

    let mut saved_tracks: Vec<SavedTrack> = Vec::new();
    let mut next_liked_track: Option<String> = None;

    let mut saved_tracks_pb = None;
    let mut fetched_tracks: u64 = 0;
    let mut total_tracks: u64 = 0;

    loop {
        let playlists_resp = fetch_saved_tracks(token.clone(), next_liked_track.clone()).await;

        let mut saved_track_response = match playlists_resp {
            Ok(playlists) => playlists,
            Err(err) => return Err(err),
        };

        saved_tracks.append(&mut saved_track_response.clone().items);
        fetched_tracks += saved_track_response.items.len() as u64;
        next_liked_track = saved_track_response.next;

        if total_tracks == 0 {
            total_tracks = saved_track_response.total;
            saved_tracks_pb = Some(ProgressBar::new(total_tracks));
        }

        if let Some(ref pb) = saved_tracks_pb {
            pb.set_position(fetched_tracks);
        }

        if next_liked_track.is_none() {
            if let Some(ref pb) = saved_tracks_pb {
                pb.finish_with_message("done");
            }
            break;
        }
    }

    Ok(saved_tracks)
}

async fn fetch_saved_tracks(
    token: String,
    next: Option<String>,
) -> Result<SavedTrackResponse, reqwest::Error> {
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let url = next
        .unwrap_or_else(|| String::from("https://api.spotify.com/v1/me/tracks?market=DE&limit=20"));
    let request_url = url.as_str();

    let response = client
        .get(request_url)
        .header("Authorization", auth_header_value)
        .send()
        .await?;

    let response_body: SavedTrackResponse = response.json().await?;

    Ok(response_body)
}
