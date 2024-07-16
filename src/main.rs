extern crate rocket; 
use db::models::Sensor;
use rocket::serde::{Deserialize, json::Json};

use diesel::prelude::*;
pub mod db;


#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SensorReading<'r> {
    pub controller_name: &'r str,
    pub api_key: &'r str,

    pub temperature: f32,
    pub humidity: f32,
    pub carbon_dioxide: f32,

    pub voc_index: i16,
    pub nox_index: i16,

    pub pm1_0: f32,
    pub pm2_5: f32,
    pub pm10: f32,   
}

#[rocket::post("/create_sensor_reading", format = "json", data = "<sensor_reading>")]
fn create_sensor_reading(sensor_reading: Json<SensorReading>) -> rocket::http::Status {    
    use db::schema::sensor_readings::dsl::*;
    use db::schema::sensors::dsl::*;

    let conn = &mut db::establish_connection();

    let sensor: Sensor = match sensors.find(sensor_reading.controller_name).first(conn) {
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
        .values(
            (
                sensor_name.eq(sensor_reading.controller_name),
    
                temperature.eq(sensor_reading.temperature),
                humidity.eq(sensor_reading.humidity),
                carbon_dioxide.eq(sensor_reading.carbon_dioxide),

                voc_index.eq(sensor_reading.voc_index),
                nox_index.eq(sensor_reading.nox_index),

                pm1_0.eq(sensor_reading.pm1_0),
                pm2_5.eq(sensor_reading.pm2_5),
                pm10.eq(sensor_reading.pm10),
            )
        )
        .execute(conn)
        .expect("Error adding reading");


    rocket::http::Status::Ok
}


#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index])
        .mount("/", rocket::routes![create_sensor_reading])
}

