use std::sync::Arc;

use juniper::FieldResult;
use juniper::RootNode;
use diesel::PgConnection;

use crate::db::DbPooledConnection;
use crate::models::thermostat_status::*;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<DbPooledConnection>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Query the current (latest) thermostat status")]
    fn thermostat_status(context: &Context) -> FieldResult<ThermostatStatus> {
        let connection: &PgConnection = &context.db;

        let result = ThermostatStatus::get_latest(connection)?;
        return Ok(result);
    }

    #[graphql(description = "Query the thermostat status history")]
    fn thermostat_status_history(context: &Context) -> FieldResult<Vec<ThermostatStatus>> {
        let connection: &PgConnection = &context.db;

        let results = ThermostatStatus::get_history(connection)?;
        return Ok(results);
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(context: &Context, data: NewThermostatStatus) -> FieldResult<ThermostatStatus> {
        let connection: &PgConnection = &context.db;

        ThermostatStatus::insert(connection, data)?;

        let result = ThermostatStatus::get_latest(connection)?;
        return Ok(result);
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {})
}

pub fn create_context(pg_pool: DbPooledConnection) -> Context {
    Context {db: Arc::new(pg_pool)}
}
