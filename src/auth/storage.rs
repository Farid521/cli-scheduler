use std::fs;
use std::path::PathBuf;
use crate::auth::token::StoredToken;
use crate::errors::AuthError;

fn token_path() -> PathBuf {
    let base = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("cli-scheduler").join("token.json")
}

pub fn save_token(token: &StoredToken) -> Result<(), AuthError> {
    let path = token_path();

    // Buat direktori jika belum ada
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AuthError::Storage(e.to_string()))?;
    }

    let json = serde_json::to_string_pretty(token)
        .map_err(|e| AuthError::Storage(e.to_string()))?;

    fs::write(&path, json)
        .map_err(|e| AuthError::Storage(e.to_string()))?;

    println!("Token disimpan di: {}", path.display());
    Ok(())
}

pub fn load_token() -> Option<StoredToken> {
    let path = token_path();
    let json = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

pub fn token_exists() -> bool {
    token_path().exists()
}
