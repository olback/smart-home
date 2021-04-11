use {
    chrono::TimeZone,
    rocket_contrib::json::Json,
    serde::{Deserialize, Serialize},
    std::time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime {
    timezone: String,
    time: String,
    date: String,
    timestamp: i64,
}

#[rocket::get("/datetime")]
pub async fn get_datetime() -> Option<Json<DateTime>> {
    crate::CONF
        .datetime
        .as_ref()
        .map(|dt| {
            let now = dt.timezone.timestamp(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                0,
            );
            DateTime {
                timezone: dt.timezone.to_string(),
                time: now.format("%H:%M:%S").to_string(),
                date: now.format("%a %b %e").to_string(),
                timestamp: now.timestamp(),
            }
        })
        .map(|dt| Json(dt))
}
