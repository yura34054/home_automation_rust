use chrono::NaiveDateTime; 

use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::db::schema::{sensors, sensor_readings};


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = sensors)]
#[diesel(primary_key(name))]
pub struct Sensor {
    pub name: String,
    pub api_key: String,
}

#[derive(Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = sensor_readings)]
#[diesel(belongs_to(Sensor))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SensorReadingInput {
    pub sensor_name: String,

    #[diesel(skip_insertion)]
    pub api_key: String,

    pub temperature: f32,
    pub humidity: f32,
    pub carbon_dioxide: f32,

    pub voc_index: i16,
    pub nox_index: i16,

    pub pm1_0: f32,
    pub pm2_5: f32,
    pub pm10: f32,
}


#[derive(Serialize, Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = sensor_readings)]
#[diesel(belongs_to(Sensor, foreign_key = sensor_name))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SensorReading {
    pub id: i32,
    pub sensor_name: String,
    pub created_on: NaiveDateTime,

    pub temperature: f32,
    pub humidity: f32,
    pub carbon_dioxide: f32,

    pub voc_index: i16,
    pub nox_index: i16,

    pub pm1_0: f32,
    pub pm2_5: f32,
    pub pm10: f32,
}
