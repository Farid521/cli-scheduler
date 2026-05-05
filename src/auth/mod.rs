pub mod token;
pub mod storage;

use crate::errors::{AuthError, CalendarError};
pub use token::StoredToken;

/// Alur autentikasi lengkap: buka browser → terima code → tukar token → simpan
pub fn start_auth_flow() -> Result<(), AuthError> {
    crate::browser::login_redirect()
        .map_err(|e| AuthError::Storage(e.to_string()))?;

    let code = crate::server::listen_for_code()
        .map_err(|e| AuthError::Storage(e.to_string()))?;

    let token = token::exchange_code_for_token(&code)?;
    storage::save_token(&token)?;

    println!("Autentikasi berhasil! Selamat datang.");
    Ok(())
}

/// Ambil token yang valid. Jika sudah expired, refresh otomatis.
/// Jika tidak ada token sama sekali, kembalikan error agar user menjalankan `auth`.
pub fn get_valid_token() -> Result<StoredToken, CalendarError> {
    let token = storage::load_token().ok_or(CalendarError::NotAuthenticated)?;

    if token.is_expired() {
        let refreshed = token::refresh_access_token(&token)
            .map_err(|e| CalendarError::ParseFailed(e.to_string()))?;
        storage::save_token(&refreshed)
            .map_err(|e| CalendarError::ParseFailed(e.to_string()))?;
        Ok(refreshed)
    } else {
        Ok(token)
    }
}
