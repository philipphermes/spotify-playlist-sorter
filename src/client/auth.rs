use crate::model::token::AccessTokenResponse;
use crate::TOKEN;
use ::url::Url;
use colored::Colorize;
use reqwest::Client;
use std::env;
use std::error::Error;
use tiny_http::{Response, Server};

pub async fn auth() {
    let login_url = get_url();

    match open::that(login_url.as_str()) {
        Ok(()) => println!("{} {}", "Opened: ".green(), login_url.blue()),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", login_url, err),
    }

    let code = match get_code().await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error getting code: {}", err);
            return;
        }
    };

    let token = match get_token(code).await {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to get token: {}", err);
            return;
        }
    };

    println!("{}", "Received Token".green());

    let mut global_token = TOKEN.lock().unwrap();
    *global_token = token;
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

async fn get_token(code: String) -> Result<String, reqwest::Error> {
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

    Ok(json.access_token)
}
