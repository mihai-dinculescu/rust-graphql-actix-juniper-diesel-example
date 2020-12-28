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
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<GraphQLRequest>>,
    data_body: Option<web::Json<GraphQLRequest>>,
    db_pool: web::Data<DbPool>,
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

            return Ok(HttpResponse::Ok().json(&err));
        }
    }

    let db_pool = (*db_pool).clone();
    let ctx = create_context(db_pool);
    let res = data.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
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
