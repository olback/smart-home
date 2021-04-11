use {
    file_responder::FileResponder,
    rocket::{http::ContentType, response::Redirect},
};

// TODO: Generate this file dynamically

// HTML
const FILE_INDEX: &'static [u8] = include_bytes!("../../../../web/dist/smart-home/index.html");

// JS
const FILE_MAIN_JS: &'static [u8] = include_bytes!("../../../../web/dist/smart-home/main.js");
const FILE_POLYFILLS_JS: &'static [u8] =
    include_bytes!("../../../../web/dist/smart-home/polyfills.js");
const FILE_RUNTIME_JS: &'static [u8] = include_bytes!("../../../../web/dist/smart-home/runtime.js");

// CSS
const FILE_STYLES_CSS: &'static [u8] = include_bytes!("../../../../web/dist/smart-home/styles.css");

// Other
const FILE_LICENCES: &'static [u8] =
    include_bytes!("../../../../web/dist/smart-home/3rdpartylicenses.txt");
const FILE_FAVICON: &'static [u8] = include_bytes!("../../../../web/dist/smart-home/favicon.ico");

mod file_responder;

#[rocket::get("/")]
pub async fn root_to_app() -> Redirect {
    Redirect::to("/app")
}

// HTML

#[rocket::get("/")]
pub async fn app_root() -> FileResponder {
    FileResponder::new(FILE_INDEX, ContentType::HTML)
}

#[rocket::get("/<_catch>")]
pub async fn index(_catch: String) -> FileResponder {
    FileResponder::new(FILE_INDEX, ContentType::HTML)
}

// JS

#[rocket::get("/main.js")]
pub async fn main_js() -> FileResponder {
    FileResponder::new(FILE_MAIN_JS, ContentType::JavaScript)
}

#[rocket::get("/polyfills.js")]
pub async fn polyfills_js() -> FileResponder {
    FileResponder::new(FILE_POLYFILLS_JS, ContentType::JavaScript)
}

#[rocket::get("/runtime.js")]
pub async fn runtime_js() -> FileResponder {
    FileResponder::new(FILE_RUNTIME_JS, ContentType::JavaScript)
}

// CSS

#[rocket::get("/styles.css")]
pub async fn styles_css() -> FileResponder {
    FileResponder::new(FILE_STYLES_CSS, ContentType::CSS)
}

// Other

#[rocket::get("/favicon.ico")]
pub async fn favicon() -> FileResponder {
    FileResponder::new(FILE_FAVICON, ContentType::Icon)
}

#[rocket::get("/3rdpartylicenses.txt")]
pub async fn licenses() -> FileResponder {
    FileResponder::new(FILE_LICENCES, ContentType::Plain)
}
