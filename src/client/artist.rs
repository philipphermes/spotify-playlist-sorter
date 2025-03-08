use reqwest::Client;
use crate::model::artist::{Artist, ArtistsResponse};
use colored::Colorize;
use indicatif::ProgressBar;

pub async fn get_artists(token: String, artist_ids: Vec<String>) -> Result<Vec<Artist>, reqwest::Error> {
    println!("\n\n{}", "Fetching artists".green().bold());

    let mut artists: Vec<Artist> = Vec::new();

    let mut fetched_artists: u64 = 0;
    let total_artists: u64 = artist_ids.len() as u64;
    let mut artists_pb = ProgressBar::new(total_artists);

    let mut artist_search = "".to_string();
    let mut added_artists_to_search = 0;

    for artist_id in artist_ids {
        if added_artists_to_search < 50 {
            added_artists_to_search += 1;

            if added_artists_to_search == 1 {
                artist_search = artist_id;
            } else {
                artist_search = [artist_search, artist_id].join(",")
            }
        } else {
            let artists_resp = fetch_artists(token.clone(), artist_search).await;

            let mut artist_response = match artists_resp {
                Ok(artists) => artists,
                Err(err) => return Err(err),
            };

            artists.append(&mut artist_response.clone().artists);
            fetched_artists += artist_response.artists.len() as u64;

            artists_pb.set_position(fetched_artists);

            artist_search = "".to_string();
            added_artists_to_search = 0;
        }
    }

    artists_pb.finish_with_message("Done");

    Ok(artists)
}

async fn fetch_artists(
    token: String,
    artist_ids: String,
) -> Result<ArtistsResponse, reqwest::Error> {
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let response = client
        .get(format!("https://api.spotify.com/v1/artists?ids={}", artist_ids))
        .header("Authorization", auth_header_value)
        .send()
        .await?;

    let response_body: ArtistsResponse = response.json().await?;

    Ok(response_body)
}