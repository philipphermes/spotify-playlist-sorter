use crate::model::artist::Artist;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SavedTrackResponse {
    pub next: Option<String>,
    pub total: i64,
    pub items: Vec<SavedTrack>,
}

#[derive(Deserialize, Debug)]
pub struct SavedTrack {
    pub track: Track,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub artists: Vec<Artist>,
}
