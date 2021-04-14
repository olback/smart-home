// API V1

pub mod datetime;
pub mod email;
pub mod github;
pub mod shoplistic;
pub mod sonos;
pub mod temphum;
pub mod tradfri;
pub mod weather;

#[rocket::get("/")]
pub async fn api_root() -> &'static str {
    "API V1 Root"
}
