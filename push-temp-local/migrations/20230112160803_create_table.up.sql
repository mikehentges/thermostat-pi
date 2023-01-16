-- Add up migration script here
-- Add migration script here
CREATE TABLE shop_thermostat(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    record_day DATE NOT NULL,
    record_date TIMESTAMPTZ NOT NULL,
    temperature NUMERIC(6, 3) NOT NULL,
    thermostat_value INTEGER NOT NULL,
    thermostat_on BOOLEAN NOT NULL
);
CREATE INDEX shop_thermostat_day ON shop_thermostat (record_day);
CREATE UNIQUE INDEX shop_thermostat_date ON shop_thermostat (record_date);