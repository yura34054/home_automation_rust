-- Your SQL goes here
CREATE TABLE `sensor_readings`(
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
    `sensor_name` VARCHAR(63) NOT NULL REFERENCES sensors(`name`),

    `created_on` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    `temperature` FLOAT NOT NULL,
    `humidity` FLOAT NOT NULL,
    `carbon_dioxide` FLOAT NOT NULL,
    `voc_index` SMALLINT NOT NULL,
    `nox_index` SMALLINT NOT NULL,
    `pm1_0` FLOAT NOT NULL,
    `pm2_5` FLOAT NOT NULL,
    `pm10` FLOAT NOT NULL
);

