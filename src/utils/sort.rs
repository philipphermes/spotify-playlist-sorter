use strsim::jaro_winkler;
use crate::model::artist::Artist;
use crate::model::playlist::Playlist;
use crate::model::saved_tracks::{SavedTrack};

pub fn sort(saved_tracks: Vec<SavedTrack>, playlists: &mut Vec<Playlist>, artists: Vec<Artist>) {
    // Merge artist details into saved_tracks
    let saved_tracks = merge_artists_to_saved_tracks(saved_tracks, artists);

    let threshold = 0.75;

    for saved_track in saved_tracks {
        categorize_track(saved_track, playlists, threshold);
    }
}

fn categorize_track(saved_track: SavedTrack, playlists: &mut Vec<Playlist>, threshold: f64) {
    // Iterate through artists and check if their genres are available
    for artist in saved_track.track.clone().artists {
        // Ensure genres are available before categorizing
        if let Some(genres) = artist.genres {
            for genre in genres {
                if let Some(index) = find_best_match(&genre, playlists, threshold) {
                    // Ensure playlist's tracks are initialized
                    if playlists[index].songs.is_none() {
                        playlists[index].songs = Some(Vec::new());
                    }
                    // Now safely push the track
                    playlists[index].songs.as_mut().unwrap().push(saved_track.track);
                    return;
                }
            }
        }
    }

    // If no match is found, add the track to the "Miscellaneous" playlist
    if let Some(index) = playlists.iter_mut().position(|p| p.name == "Miscellaneous") {
        // Ensure playlist's tracks are initialized
        if playlists[index].songs.is_none() {
            playlists[index].songs = Some(Vec::new());
        }
        // Now safely push the track
        playlists[index].songs.as_mut().unwrap().push(saved_track.track);
    }
}

fn merge_artists_to_saved_tracks(mut saved_tracks: Vec<SavedTrack>, artists: Vec<Artist>) -> Vec<SavedTrack> {
    for saved_track in &mut saved_tracks {
        for saved_artist in &mut saved_track.track.artists {
            if let Some(artist) = artists.iter().find(|artist| artist.id == saved_artist.id) {
                *saved_artist = artist.clone();
            }
        }
    }
    saved_tracks
}

fn find_best_match(genre: &str, playlists: &Vec<Playlist>, threshold: f64) -> Option<usize> {
    let mut best_match: Option<usize> = None;
    let mut highest_score = 0.0;

    for (i, playlist) in playlists.iter().enumerate() {
        let similarity = jaro_winkler(genre, &playlist.name);
        if similarity > highest_score && similarity >= threshold {
            highest_score = similarity;
            best_match = Some(i);
        }
    }

    best_match
}
