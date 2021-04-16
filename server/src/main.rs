mod config;
mod routes;

lazy_static::lazy_static! {
    pub static ref CONF: config::Config = config::Config::load("Server.toml").expect("Failed to read/parse config");
}

#[rocket_contrib::database("smart-home")]
pub struct DbCon(rocket_contrib::databases::postgres::Client);

#[rocket::launch]
async fn rocket() -> rocket::Rocket {
    println!("{:#?}", *CONF);

    rocket::custom(CONF.rocket())
        .mount(
            "/",
            rocket::routes![routes::app::root_to_app, routes::api::api_root],
        )
        .mount(
            "/app",
            rocket::routes![
                // HTML
                routes::app::app_root,
                routes::app::index,
                // JS
                routes::app::main_js,
                routes::app::polyfills_js,
                routes::app::runtime_js,
                // CSS
                routes::app::styles_css,
                // Other
                routes::app::favicon,
                routes::app::licenses
            ],
        )
        .mount(
            "/api/v1",
            rocket::routes![
                // GET
                routes::api::v1::api_root,
                routes::api::v1::datetime::get_datetime,
                // routes::api::v1::email::get_email,
                // routes::api::v1::github::get_github,
                // routes::api::v1::shoplistic::get_shoplistic,
                // routes::api::v1::sonos::get_sonos,
                // routes::api::v1::tradfri::get_tradfri,
                // routes::api::v1::weather::get_weather,
                // POST
                routes::api::v1::temphum::post_temphum
            ],
        )
        .attach(DbCon::fairing())
}
