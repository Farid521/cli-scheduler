use serde::{Deserialize, Serialize};

/// Representasi sebuah event di Google Calendar
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarEvent {
    pub id: Option<String>,
    pub summary: String,
    pub description: Option<String>,
    pub start: EventDateTime,
    pub end: EventDateTime,
}

/// Waktu mulai/selesai event (format RFC3339 atau tanggal saja)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventDateTime {
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl EventDateTime {
    /// Buat EventDateTime dari string format "YYYY-MM-DDTHH:MM"
    pub fn from_local_str(s: &str) -> Self {
        // Tambahkan :00+07:00 jika belum ada timezone
        let dt = if s.contains('+') || s.ends_with('Z') {
            s.to_string()
        } else {
            format!("{}:00+07:00", s)
        };
        EventDateTime {
            date_time: Some(dt),
            date: None,
            time_zone: Some("Asia/Jakarta".to_string()),
        }
    }
}

/// Response dari Google Calendar API untuk list events
#[derive(Debug, Deserialize)]
pub struct EventListResponse {
    pub items: Vec<CalendarEvent>,
}
