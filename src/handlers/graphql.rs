use std::sync::Arc;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::http::header::HeaderMap;
use juniper::http::{GraphQLRequest, playground::playground_source};
use juniper::serde::ser::Error as SerdeError;

use crate::db::DbPool;
use crate::schema_graphql::{SchemaGraphQL, create_context};
use crate::models::key::Key;
use crate::models::errors::GraphQLErrors;

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data: web::Json<GraphQLRequest>,
    pool: web::Data<DbPool>,
    key: web::Data<Key>
) -> Result<HttpResponse, Error> {
    let headers = req.headers();

    // let instrospection queries through
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
        let db_pool = pool
            .get()
            .map_err(|e| {
                serde_json::error::Error::custom(e)
            })?;

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
            let value = value.to_str().map_err(|e| {serde_json::error::Error::custom(e) }).unwrap();

            if value != key.value {
                return Err("Invalid Key");
            }
        },
        None => {
            return Err("Missing header: key");
        }
    }

    Ok(())
}
