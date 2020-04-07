use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

use diesel::prelude::*;

use crate::schema::thermostat_status;

#[derive(GraphQLObject, Queryable, Clone)]
#[graphql(description = "Thermostat status")]
pub struct ThermostatStatus {
    id: i32,
    status: bool,
    timestamp: NaiveDateTime,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "thermostat_status"]
#[graphql(description = "New thermostat status")]
pub struct NewThermostatStatus {
    status: bool,
}

impl ThermostatStatus {
    pub fn get_latest(connection: &PgConnection) -> QueryResult<ThermostatStatus> {
        use crate::schema::thermostat_status::dsl::*;

        thermostat_status
            .order(timestamp.desc())
            .limit(1)
            .get_result(connection)
    }

    pub fn get_history(connection: &PgConnection) -> QueryResult<Vec<ThermostatStatus>> {
        use crate::schema::thermostat_status::dsl::*;

        thermostat_status
            .order(timestamp.desc())
            .limit(20)
            .load::<ThermostatStatus>(connection)
    }

    pub fn insert(connection: &PgConnection, data: NewThermostatStatus) -> QueryResult<usize> {
        diesel::insert_into(thermostat_status::table)
            .values(&data)
            .execute(connection)
    }
}
