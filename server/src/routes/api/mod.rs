use rocket::response::Redirect;

pub mod v1;

#[rocket::get("/api")]
pub async fn api_root() -> Redirect {
    Redirect::to("/api/v1")
}
