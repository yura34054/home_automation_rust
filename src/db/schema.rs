// @generated automatically by Diesel CLI.

diesel::table! {
    sensor_readings (id) {
        id -> Int4,
        #[max_length = 63]
        sensor_name -> Varchar,
        created_on -> Timestamp,
        temperature -> Float4,
        humidity -> Float4,
        carbon_dioxide -> Float4,
        voc_index -> Int2,
        nox_index -> Int2,
        pm1_0 -> Float4,
        pm2_5 -> Float4,
        pm10 -> Float4,
    }
}

diesel::table! {
    sensors (name) {
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 255]
        api_key -> Varchar,
    }
}

diesel::joinable!(sensor_readings -> sensors (sensor_name));

diesel::allow_tables_to_appear_in_same_query!(
    sensor_readings,
    sensors,
);
