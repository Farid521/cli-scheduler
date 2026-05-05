use reqwest::blocking::{Client, RequestBuilder};
use crate::auth::StoredToken;

const CALENDAR_BASE_URL: &str = "https://www.googleapis.com/calendar/v3";

/// HTTP client wrapper yang otomatis menambahkan Bearer token
pub struct CalendarClient {
    client: Client,
    token: StoredToken,
}

impl CalendarClient {
    pub fn new(token: StoredToken) -> Self {
        CalendarClient {
            client: Client::new(),
            token,
        }
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", CALENDAR_BASE_URL, path);
        self.client
            .get(&url)
            .bearer_auth(&self.token.access_token)
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", CALENDAR_BASE_URL, path);
        self.client
            .post(&url)
            .bearer_auth(&self.token.access_token)
    }

    pub fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", CALENDAR_BASE_URL, path);
        self.client
            .delete(&url)
            .bearer_auth(&self.token.access_token)
    }
}
