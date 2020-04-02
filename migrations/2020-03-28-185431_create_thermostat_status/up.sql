CREATE TABLE thermostat_status (
    id SERIAL PRIMARY KEY,
    status BOOLEAN NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO thermostat_status(id, status) VALUES (1, false);
