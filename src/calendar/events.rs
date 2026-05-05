use crate::calendar::client::CalendarClient;
use crate::calendar::models::{CalendarEvent, EventDateTime, EventListResponse};
use crate::errors::CalendarError;

/// Tambahkan event baru ke Google Calendar
pub fn create_event(
    client: &CalendarClient,
    title: &str,
    start: &str,
    end: &str,
    description: Option<&str>,
) -> Result<CalendarEvent, CalendarError> {
    let event = CalendarEvent {
        id: None,
        summary: title.to_string(),
        description: description.map(|s| s.to_string()),
        start: EventDateTime::from_local_str(start),
        end: EventDateTime::from_local_str(end),
    };

    let res = client
        .post("/calendars/primary/events")
        .json(&event)
        .send()?;

    if !res.status().is_success() {
        let msg = res.text().unwrap_or_else(|_| "unknown error".to_string());
        return Err(CalendarError::ParseFailed(msg));
    }

    let created: CalendarEvent = res.json()?;
    Ok(created)
}

/// Tampilkan daftar event mendatang dari kalender utama
pub fn list_events(client: &CalendarClient) -> Result<Vec<CalendarEvent>, CalendarError> {
    let now = chrono::Utc::now().to_rfc3339();

    let res = client
        .get("/calendars/primary/events")
        .query(&[
            ("orderBy", "startTime"),
            ("singleEvents", "true"),
            ("timeMin", &now),
            ("maxResults", "10"),
        ])
        .send()?;

    if !res.status().is_success() {
        let msg = res.text().unwrap_or_else(|_| "unknown error".to_string());
        return Err(CalendarError::ParseFailed(msg));
    }

    let data: EventListResponse = res.json()?;
    Ok(data.items)
}

/// Hapus event berdasarkan ID
pub fn delete_event(client: &CalendarClient, event_id: &str) -> Result<(), CalendarError> {
    let path = format!("/calendars/primary/events/{}", event_id);

    let res = client.delete(&path).send()?;

    match res.status().as_u16() {
        204 => Ok(()),
        404 => Err(CalendarError::EventNotFound(event_id.to_string())),
        _ => {
            let msg = res.text().unwrap_or_else(|_| "unknown error".to_string());
            Err(CalendarError::ParseFailed(msg))
        }
    }
}
