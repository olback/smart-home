// API V1

#[rocket::get("/")]
pub async fn api_root() -> &'static str {
    "API V1 Root"
}
