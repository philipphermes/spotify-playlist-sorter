use serde::{Deserialize, Serialize};
use crate::model::saved_tracks::Track;

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistResponse {
    pub next: Option<String>,
    pub total: u64,
    pub items: Vec<Playlist>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub owner: Owner,
    pub songs: Option<Vec<Track>>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Owner {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistCreateResponse {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistAddResponse {
    pub snapshot_id: String,
}