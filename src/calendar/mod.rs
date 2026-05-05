pub mod client;
pub mod models;
pub mod events;

use crate::auth;
use crate::errors::CalendarError;
use client::CalendarClient;

/// Helper: dapatkan client yang sudah diautentikasi
pub fn get_client() -> Result<CalendarClient, CalendarError> {
    let token = auth::get_valid_token()?;
    Ok(CalendarClient::new(token))
}

pub use events::{create_event, list_events, delete_event};
pub use models::{CalendarEvent, EventDateTime};
