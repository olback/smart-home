use crate::DbCon;

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
pub async fn post_temphum(data: Form<TempHum>, db: DbCon) -> Status {
    let temp_hum = data.into_inner();
    println!("{:#?}", temp_hum);
    match db
        .run::<_, Result<(), rocket_contrib::databases::postgres::Error>>(|con| {
            let temp_hum = temp_hum;
            con.execute(
                "select insert_temp($1, $2)",
                &[&temp_hum.location, &temp_hum.temperature],
            )?;
            con.execute(
                "select insert_hum($1, $2)",
                &[&temp_hum.location, &temp_hum.relative_humidity],
            )?;
            Ok(())
        })
        .await
    {
        Ok(_) => Status::Created,
        Err(e) => {
            eprintln!("{:#?}", e);
            Status::InternalServerError
        }
    }
}
