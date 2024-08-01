extern crate rocket; 
use rocket_dyn_templates::Template;

pub mod db;
mod transport;

use transport::api;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![
            transport::sensor_charts,
            transport::index
        ])
        .mount("/api/sensor_reading", rocket::routes![
            api::sensor_reading::create_sensor_reading, 
            api::sensor_reading::get_sensor_readings_from_seconds,
            api::sensor_reading::get_sensor_readings_from_id,
        ])
        .mount("/public", rocket::fs::FileServer::from("./static"))
        .attach(Template::fairing())
}
