use crate::model::artist::Artist;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistResponse {
    pub next: Option<String>,
    pub total: u64,
    pub items: Vec<Playlist>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub owner: Owner,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Owner {
    pub id: String,
}
