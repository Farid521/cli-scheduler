use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Kode otorisasi tidak ditemukan.")]
    MissingCode,

    #[error("Izin Google Calendar tidak diberikan. Aplikasi tidak bisa berfungsi tanpa izin tersebut.")]
    ScopesMissing,

    #[error("Pertukaran token gagal: {0}")]
    TokenExchangeFailed(String),

    #[error("Kesalahan variabel lingkungan: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Kesalahan HTTP request: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Kesalahan baca/tulis token: {0}")]
    Storage(String),
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Kesalahan IO Server: {0}")]
    Io(#[from] std::io::Error),

    #[error("Server timeout: tidak ada koneksi dalam batas waktu.")]
    Timeout,

    #[error("Kesalahan autentikasi pada server: {0}")]
    Auth(#[from] AuthError),
}

#[derive(Debug, Error)]
pub enum CalendarError {
    #[error("Kesalahan HTTP request: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Gagal parse response API: {0}")]
    ParseFailed(String),

    #[error("Token tidak ditemukan. Jalankan `auth` terlebih dahulu.")]
    NotAuthenticated,

    #[error("Event dengan ID '{0}' tidak ditemukan.")]
    EventNotFound(String),
}

/// Error umum tingkat aplikasi
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error("{0}")]
    Server(#[from] ServerError),

    #[error("{0}")]
    Calendar(#[from] CalendarError),

    #[error("Kesalahan IO: {0}")]
    Io(#[from] std::io::Error),
}
