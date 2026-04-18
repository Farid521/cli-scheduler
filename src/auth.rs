use reqwest::blocking::Client;
use std;
use serde::Serialize;
use std::env;

#[derive(Debug)]
pub enum AuthError {
    MissingCode,
    EnvVar(std::env::VarError),
    Request(reqwest::Error),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingCode => write!(f, "auth code is not provided"),
            AuthError::EnvVar(e) => write!(f, "environment variable error: {}", e),
            AuthError::Request(e) => write!(f, "request error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AuthError::EnvVar(e) => Some(e),
            AuthError::Request(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::env::VarError> for AuthError {
    fn from(err: std::env::VarError) -> Self {
        AuthError::EnvVar(err)
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(err: reqwest::Error) -> Self {
        AuthError::Request(err)
    }
}

#[derive(Serialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

pub fn get_token(code: &str) -> Result<(), AuthError>{
    if code == "null" {
        return Err(AuthError::MissingCode);
    } 

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