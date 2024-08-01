use diesel::prelude::*;

use crate::*;

use chrono::{TimeDelta, Utc};
use db::models::{Sensor, SensorReading, SensorReadingInput};
use rocket::serde::json::Json;


#[rocket::post("/create_sensor_reading", format = "json", data = "<sensor_reading>")]
pub fn create_sensor_reading(sensor_reading: Json<SensorReadingInput>) -> rocket::http::Status {    
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


#[rocket::get("/from_id/<sensor>/<reading_id>")]
pub fn get_sensor_readings_from_id(sensor: String, reading_id: Option<i32>) ->  Json<Vec<SensorReading>> {
    use db::schema::sensor_readings::dsl::*;
    let conn = &mut db::establish_connection();

    let reading_id: i32 = match reading_id {
        Some(reading_id) => reading_id,
        None => return Json(Vec::new()),
    };

    let data = sensor_readings
        .filter(id.gt(reading_id))
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


#[rocket::get("/from_seconds/<sensor>/<seconds>")]
pub fn get_sensor_readings_from_seconds(sensor: String, seconds: Option<i64>) ->  Json<Vec<SensorReading>> {
    use db::schema::sensor_readings::dsl::*;
    let conn = &mut db::establish_connection();

    let data = sensor_readings
        .filter(
            created_on.ge(
                Utc::now()
                .naive_utc()
                .checked_sub_signed(
                    TimeDelta::try_seconds(seconds.unwrap_or(2*3600)).unwrap()
                )
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
