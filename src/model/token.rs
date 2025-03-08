use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}
