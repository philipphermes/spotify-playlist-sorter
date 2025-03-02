use reqwest::Client;
use std::env;

pub async fn get_playlist(token: String) ->  Result<String, Box<dyn std::error::Error>> {
    let user_id = env::var("USER_ID").expect("USER_ID not set");
    let auth_header_value = format!("Bearer {}", token);
    let client = Client::new();

    let response = client
        .get( format!("https://api.spotify.com/v1/users/{}/playlists", user_id))
        .header("Authorization", auth_header_value)
        .send()
        .await?;

    let status = response.status();
    let response_body = response.text().await?;

    if status.is_success() {
        Ok(response_body)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch token")))
    }
}
