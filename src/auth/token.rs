use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::errors::AuthError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    /// Unix timestamp (detik) saat token expired
    pub expires_at: i64,
}

impl StoredToken {
    /// Kembalikan true jika token sudah expired atau akan expired dalam 60 detik
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        now >= self.expires_at - 60
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    scope: String,
    token_type: String,
    /// Berapa detik token berlaku sejak diterbitkan
    expires_in: Option<u64>,
}

fn compute_expires_at(expires_in: Option<u64>) -> i64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    (now + expires_in.unwrap_or(3600)) as i64
}

#[derive(Serialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

#[derive(Serialize)]
struct RefreshRequest {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    grant_type: String,
}

pub fn exchange_code_for_token(code: &str) -> Result<StoredToken, AuthError> {
    let client_id = env::var("CLIENT_ID")?;
    let redirect_uri = env::var("REDIRECT_URI")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let body = TokenRequest {
        client_id,
        client_secret,
        code: code.to_string(),
        grant_type: "authorization_code".to_string(),
        redirect_uri,
    };

    let client = Client::new();
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&body)
        .send()?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_else(|_| "unknown error".to_string());
        return Err(AuthError::TokenExchangeFailed(err_text));
    }

    let data: TokenResponse = res.json()?;

    // Validasi scope Google Calendar
    let required = "https://www.googleapis.com/auth/calendar";
    if !data.scope.contains(required) {
        return Err(AuthError::ScopesMissing);
    }

    let refresh_token = data
        .refresh_token
        .ok_or_else(|| AuthError::Storage("Refresh token tidak diterima dari Google.".to_string()))?;

    Ok(StoredToken {
        access_token: data.access_token,
        refresh_token,
        scope: data.scope,
        token_type: data.token_type,
        expires_at: compute_expires_at(data.expires_in),
    })
}

pub fn refresh_access_token(stored: &StoredToken) -> Result<StoredToken, AuthError> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let body = RefreshRequest {
        client_id,
        client_secret,
        refresh_token: stored.refresh_token.clone(),
        grant_type: "refresh_token".to_string(),
    };

    let client = Client::new();
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&body)
        .send()?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_else(|_| "unknown error".to_string());
        return Err(AuthError::TokenExchangeFailed(err_text));
    }

    let data: TokenResponse = res.json()?;

    Ok(StoredToken {
        access_token: data.access_token,
        refresh_token: data.refresh_token.unwrap_or_else(|| stored.refresh_token.clone()),
        scope: data.scope,
        token_type: data.token_type,
        expires_at: compute_expires_at(data.expires_in),
    })
}
