extern crate rocket; 

use chrono::{TimeDelta, Utc};
use db::models::{Sensor, SensorReading, SensorReadingInput};
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};

use diesel::prelude::*;
pub mod db;


#[rocket::get("/<sensor>")]
fn index(sensor: String) -> Template {
    use db::schema::sensor_readings::dsl::*;
    let conn = &mut db::establish_connection();

    let seconds: i64 = 2*3600;
    let data = sensor_readings
        .filter(
            created_on.ge(
                Utc::now()
                .naive_utc()
                .checked_sub_signed(TimeDelta::try_seconds(seconds).unwrap())
                .unwrap()
            )
        )
        .filter(sensor_name.eq(sensor))
        .load::<SensorReading>(conn);

    let data = match data {
        Ok(data) => data,
        Err(err) => match err {
            diesel::result::Error::NotFound => vec![],
            _ => panic!("Database error - {}", err),
        }
    };

    Template::render("index", context! { 
        data: data, 
    })
}


#[rocket::post("/create_sensor_reading", format = "json", data = "<sensor_reading>")]
fn create_sensor_reading(sensor_reading: Json<SensorReadingInput>) -> rocket::http::Status {    
    use db::schema::sensor_readings::dsl::*;
    use db::schema::sensors::dsl::*;

    let conn = &mut db::establish_connection();

    let sensor: Sensor = match sensors.find(&sensor_reading.sensor_name).first(conn) {
        Ok(sensor) => sensor,
        Err(err) => match err {
            diesel::result::Error::NotFound => return rocket::http::Status::NotFound,
            _ => panic!("Database error - {}", err),
        }

    };
    
    if sensor.api_key != sensor_reading.api_key {
        return rocket::http::Status::Unauthorized
    };

    diesel::insert_into(sensor_readings)
        .values(&*sensor_reading)
        .execute(conn)
        .expect("Error adding reading");

    rocket::http::Status::Ok
}


#[rocket::get("/get_sensor_readings/<sensor>/<seconds>")]
fn get_sensor_readings(sensor: String, seconds: Option<i64>) ->  Json<Vec<SensorReading>> {
    use db::schema::sensor_readings::dsl::*;
    let conn = &mut db::establish_connection();

    let seconds: i64 = match seconds {
        Some(seconds) => seconds,
        None => 2*3600,
    };

    let data = sensor_readings
        .filter(
            created_on.ge(
                Utc::now()
                .naive_utc()
                .checked_sub_signed(TimeDelta::try_seconds(seconds).unwrap())
                .unwrap()
            )
        )
        .filter(sensor_name.eq(sensor))
        .load::<SensorReading>(conn);

    let data = match data {
        Ok(data) => data,
        Err(err) => match err {
            diesel::result::Error::NotFound => return Json(Vec::new()),
            _ => panic!("Database error - {}", err),
        }
    };

    Json(data)  
}


#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index])
        .mount("/api", rocket::routes![create_sensor_reading, get_sensor_readings])
        .attach(Template::fairing())
}
