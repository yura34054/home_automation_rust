pub mod api;

use db::models::SensorReading;
use rocket_dyn_templates::{Template, context};

use crate::*;

use diesel::prelude::*;

/*


#[rocket::get("/")]
fn index() -> Template {
    use db::schema::sensor_readings::dsl::*;

    let conn = &mut db::establish_connection();

    let readings = sensor_readings
        .group_by(sensor_name)
        .select(diesel::dsl::max(created_on))
        .load::<(Option<NaiveDateTime>)>(conn);


    let readings = match readings {
        Ok(readings) => readings,
        Err(err) => match err {
            diesel::result::Error::NotFound => vec![],
            _ => panic!("Database error - {}", err),
        }
    };

    Template::render("index", context! {readings: readings})
}
*/

#[rocket::get("/")]
pub fn index() -> Template {
    use db::schema::sensors;
    use db::schema::sensor_readings;

    let conn = &mut db::establish_connection();

    let sensors_all = sensors::table
        .select(sensors::name)
        .load::<String>(conn);

    let sensors_all = match sensors_all {
        Ok(sensors_all) => sensors_all,
        Err(err) => match err {
            diesel::result::Error::NotFound => vec![],
            _ => panic!("Database error - {}", err),
        }
    };

/*
    This creates an N+1 problem
    I would need to migrate the db to postgress to implement a 
    "not totally awful solution performance wise"

    Here's what the query will be in case I forget:
    https://stackoverflow.com/a/34715134   
*/
    let mut readings = vec![];

    for sensor in sensors_all {
        let reading = sensor_readings::table
            .filter(sensor_readings::sensor_name.eq(sensor))
            .first::<SensorReading>(conn);

        let reading = match reading {
            Ok(reading) => reading,
            Err(err) => match err {
                diesel::result::Error::NotFound => continue,
                _ => panic!("Database error - {}", err),
            }
        };

        readings.push(reading)
    };

    Template::render("index", context! {readings: readings})
}


#[rocket::get("/<sensor>")]
pub fn sensor_charts(sensor: String) -> Template {
    Template::render("charts", context! {sensor_name: sensor})
}


