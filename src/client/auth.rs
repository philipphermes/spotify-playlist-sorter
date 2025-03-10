use crate::model::token::AccessTokenResponse;
use crate::TOKEN;
use ::url::Url;
use colored::Colorize;
use reqwest::Client;
use std::env;
use std::error::Error;
use std::fs::File;
use tiny_http::{Response, Server};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct TokenData {
    access_token: String,
    expiration_time: u64,
}

pub async fn auth() {
    if let Some(token_data) = read_token_from_file() {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        if current_time < token_data.expiration_time {
            println!("\n\n{}", "Found valid token".green().bold());

            let mut global_token = TOKEN.lock().unwrap();
            *global_token = token_data.access_token;

            return;
        }

        println!("\n\n{}", "Token expired. Fetching a new one...".yellow().bold());
    }

    let login_url = get_url();

    match open::that(login_url.as_str()) {
        Ok(()) => println!("\n\n{} {}", "Opened: ".green().bold(), login_url.blue()),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", login_url, err),
    }

    let code = match get_code().await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error getting code: {}", err);
            return;
        }
    };

    let (access_token, expires_in) = match get_token(code).await {
        Ok((access_token, expires_in)) => (access_token, expires_in),
        Err(err) => {
            eprintln!("\n\nFailed to get token: {}", err);
            return;
        }
    };

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let expiration_time = current_time + expires_in;

    let token_data = TokenData {
        access_token: access_token.clone(),
        expiration_time,
    };

    if let Err(err) = write_token_to_file(&token_data) {
        eprintln!("\n\nFailed to write token to file: {}", err);
        return;
    }

    println!("\n\n{}", "Received new token".green().bold());

    let mut global_token = TOKEN.lock().unwrap();
    *global_token = access_token;
}

fn read_token_from_file() -> Option<TokenData> {
    let mut contents = String::new();

    if let Ok(mut file) = File::open("token.json") {
        if file.read_to_string(&mut contents).is_ok() {
            if !contents.is_empty() {
                if let Ok(token_data) = serde_json::from_str::<TokenData>(&contents) {
                    return Some(token_data);
                }
            }
        }
    }

    None
}

fn write_token_to_file(token_data: &TokenData) -> std::io::Result<()> {
    let json = serde_json::to_string(&token_data).unwrap();
    let mut file = File::create("token.json")?;
    file.write_all(json.as_bytes())
}

pub fn get_url() -> String {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");
    let scopes = env::var("SCOPES").expect("SCOPES not set");

    let mut url = Url::parse("https://accounts.spotify.com/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("scope", &scopes);

    url.as_str().to_string()
}

async fn get_code() -> Result<String, Box<dyn Error + Send + Sync>> {
    let server = Server::http("127.0.0.1:8888")?;
    let mut code = String::new();

    for request in server.incoming_requests() {
        if let Some(query) = request.url().split_once("?") {
            if let Some(data) = query.1.split_once("=") {
                code = data.1.to_string();

                let response = Response::from_string("You can now close this tab!");
                request.respond(response)?;
                break;
            }
        }
    }

    if code.is_empty() {
        return Err("Failed to extract authorization code".into());
    }

    Ok(code)
}

async fn get_token(code: String) -> Result<(String, u64), reqwest::Error> {
    let url = "https://accounts.spotify.com/api/token";
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    let form = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", &redirect_uri),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let client = Client::new();
    let response = client.post(url).form(&form).send().await?;

    let json: AccessTokenResponse = response.json().await?;

    Ok((json.access_token, json.expires_in))
}
