-- Your SQL goes here
CREATE TABLE sensor_readings(
    id SERIAL PRIMARY KEY,
    sensor_name VARCHAR(63) NOT NULL,
    FOREIGN KEY(sensor_name) REFERENCES sensors(name),

    created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    temperature REAL NOT NULL,
    humidity REAL NOT NULL,
    carbon_dioxide REAL NOT NULL,
    voc_index SMALLINT NOT NULL,
    nox_index SMALLINT NOT NULL,
    pm1_0 REAL NOT NULL,
    pm2_5 REAL NOT NULL,
    pm10 REAL NOT NULL
);
