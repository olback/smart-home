// "location={location}&name={name}&temperature={temperature}&relative_humidity={relative_humidity}",

use rocket::{
    form::{Form, FromForm},
    http::Status,
};

#[derive(Debug, FromForm)]
pub struct TempHum {
    location: String,
    temperature: f32,
    relative_humidity: f32,
}

#[rocket::post("/temphum", data = "<data>")]
pub async fn post_temphum(data: Form<TempHum>) -> Status {
    let temp_hum = data.into_inner();
    println!("{:#?}", temp_hum);
    Status::Created
}
