use std::sync::Arc;

use actix_web::http::header::HeaderMap;
use actix_web::http::Method;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::serde::ser::Error as SerdeError;

use crate::db::DbPool;
use crate::models::errors::GraphQLErrors;
use crate::models::key::Key;
use crate::schema_graphql::{create_context, SchemaGraphQL};

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<GraphQLRequest>>,
    data_body: Option<web::Json<GraphQLRequest>>,
    pool: web::Data<DbPool>,
    key: web::Data<Key>,
) -> Result<HttpResponse, Error> {
    let headers = req.headers();

    // fetch data from
    // query string if this is a GET
    // body if this is a POST
    let data = match *req.method() {
        Method::GET => data_query.unwrap().into_inner(),
        _ => data_body.unwrap().into_inner(),
    };

    // let introspection queries through
    if data.operation_name() != Some("IntrospectionQuery") {
        // validate key for all other requests
        if let Err(e) = validate_key(&headers, key.get_ref()) {
            let err = GraphQLErrors::new(e);

            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&err).unwrap()));
        }
    }

    let body = web::block(move || {
        let db_pool = pool.get().map_err(serde_json::error::Error::custom)?;

        let ctx = create_context(db_pool);
        let res = data.execute(&st, &ctx);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn validate_key<'a>(headers: &'a HeaderMap, key: &'a Key) -> Result<(), &'a str> {
    match headers.get("key") {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(serde_json::error::Error::custom)
                .unwrap();

            if value != key.value {
                return Err("Invalid Key");
            }
        }
        None => {
            return Err("Missing header: key");
        }
    }

    Ok(())
}
