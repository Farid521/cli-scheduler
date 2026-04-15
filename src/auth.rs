use reqwest::blocking::Client;
use std;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

fn get_token(code: &str) -> Result<(), Box<dyn std::error::Error>>{
    let res_client = Client::new();

    let client_id = env::var("CLIENT_ID")?;
    let redirect_uri = env::var("REDIRECT_URI")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let request_header = TokenRequest {
        client_id,
        client_secret,
        code: code.to_string(),
        grant_type: "authorization_code".to_string(),
        redirect_uri
    };

    // let mut auth_header = std::collections::HashMap::new();
    // auth_header.insert("code", code);
    // auth_header.insert("client_id", &client_id);
    // auth_header.insert("client_secret", &client_secret);
    // auth_header.insert("redirect_uri", &redirect_url);
    // auth_header.insert("grant_type", "authorization_code");

    let res = res_client
        .post("https://oauth2.googleapis.com/token")
        .form(&request_header)
        .send()?;
    println!("response: {}", res.text()?);
    Ok(())
}