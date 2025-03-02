use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub genres: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArtistsResponse {
    pub artists: Vec<Artist>,
}