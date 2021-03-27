mod config;
mod routes;

#[rocket_contrib::database("smart-home")]
pub struct DbCon(rocket_contrib::databases::postgres::Client);

#[rocket::launch]
async fn rocket() -> rocket::Rocket {
    let conf = config::Config::load("Server.toml").expect("Failed to read/parse config");
    rocket::custom(conf.rocket())
        .mount(
            "/",
            rocket::routes![routes::app::root_to_app, routes::api::api_root],
        )
        .mount("/api/v1", rocket::routes![routes::api::v1::api_root])
        .attach(DbCon::fairing())
}
