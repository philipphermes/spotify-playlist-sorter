use crate::model::artist::Artist;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SavedTrackResponse {
    pub next: Option<String>,
    pub total: u64,
    pub items: Vec<SavedTrack>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SavedTrack {
    pub track: Track,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Track {
    pub id: String,
    pub artists: Vec<Artist>,
}
