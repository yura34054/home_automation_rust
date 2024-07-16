// @generated automatically by Diesel CLI.

diesel::table! {
    sensor_readings (id) {
        id -> Nullable<Integer>,
        created_on -> Nullable<Timestamp>,
        temperature -> Float,
        humidity -> Float,
        carbon_dioxide -> Float,
        voc_index -> SmallInt,
        nox_index -> SmallInt,
        pm1_0 -> Float,
        pm2_5 -> Float,
        pm10 -> Float,
        sensor_name -> Text,
    }
}

diesel::table! {
    sensors (name) {
        name -> Text,
        api_key -> Text,
    }
}

diesel::joinable!(sensor_readings -> sensors (sensor_name));

diesel::allow_tables_to_appear_in_same_query!(
    sensor_readings,
    sensors,
);
