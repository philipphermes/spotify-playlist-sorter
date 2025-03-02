use reqwest::Client;
use crate::model::saved_tracks::SavedTrackResponse;

pub async fn get_saved_tracks(token: String) -> Result<SavedTrackResponse, Box<dyn std::error::Error>> {
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let response = client
        .get("https://api.spotify.com/v1/me/tracks?market=DE")
        .header("Authorization", auth_header_value)
        .send()
        .await?;

    let status = response.status();

    let response_body: SavedTrackResponse = response.json().await?;

    if status.is_success() {
        Ok(response_body)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch token",
        )))
    }
}
