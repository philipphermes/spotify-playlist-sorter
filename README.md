# Spotify Playlist Sorter

Sorts playlists after genres

## Usage

### Create App in Spotify

1. Go to [Spotify Dashboard](https://developer.spotify.com/dashboard)
2. Create app
3. Select Web API
4. Add `http://localhost:8888/callback` to your Redirect URIs

### Setup

1. Copy .env.dist to .env
    ```shell
    cp .env.dist .env
    ```
2. Paste in the CLIENT_ID AND CLIENT_SECRET from the App you just created
3. Paste in your USER_ID which can be found on your profile page here: `https://open.spotify.com/user/{your_user_id}`

### Run

```shell
cargo run
```

## Dependencies

* reqwest
* tokio
* serde
* dotenv
* url
* open
* tiny_http
* colored
* lazy_static