use std::time::SystemTime;
use diesel::prelude::*;

use crate::db::schema::{sensors, sensor_readings};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = sensors)]
#[diesel(primary_key(name))]
pub struct Sensor {
    pub name: String,
    pub api_key: String,
}


#[derive(Queryable, Identifiable)]
#[diesel(table_name = sensor_readings)]
#[diesel(belongs_to(Sensor))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SensorReading {
    pub id: i32,
    pub sensor_name: String,
    pub created_on: SystemTime,

    pub temperature: f32,
    pub humidity: f32,
    pub carbon_dioxide: f32,

    pub voc_index: i16,
    pub nox_index: i16,

    pub pm1_0: f32,
    pub pm2_5: f32,
    pub pm10: f32,
}
