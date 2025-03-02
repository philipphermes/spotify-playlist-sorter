use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String
}