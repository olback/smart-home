use rocket::response::Redirect;

#[rocket::get("/")]
pub async fn root_to_app() -> Redirect {
    Redirect::to("/app")
}
