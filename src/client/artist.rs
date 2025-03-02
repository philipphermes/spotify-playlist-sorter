use std::collections::HashMap;
use reqwest::Client;
use crate::model::artist::{Artist, ArtistsResponse};

pub async fn get_artists(token: String, artists: Vec<Artist>) -> Result<ArtistsResponse, Box<dyn std::error::Error>> {
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let mut artist_id_counts: HashMap<String, u32> = HashMap::new();
    let mut artist_ids  = "".to_string();

    for artist in artists {
        *artist_id_counts.entry(artist.id.clone()).or_insert(0) += 1;

        if !artist_ids.contains(&artist.id) {
            if !artist_ids.is_empty() {
                artist_ids.push(',');
            }
            artist_ids.push_str(&artist.id);
        }
    }

    let response = client
        .get(format!("https://api.spotify.com/v1/artists?ids={}", artist_ids))
        .header("Authorization", auth_header_value)
        .send()
        .await?;

    let status = response.status();

    let response_body: ArtistsResponse = response.json().await?;

    if status.is_success() {
        Ok(response_body)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch token",
        )))
    }
}
