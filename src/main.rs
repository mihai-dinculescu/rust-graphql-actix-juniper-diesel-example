use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use diesel_migrations::run_pending_migrations;
use dotenv::dotenv;

use ::lib::db::{establish_connection, DatabaseKind};
use ::lib::handlers::graphql::{graphql, playground};
use ::lib::models::key::Key;
use ::lib::schema_graphql::create_schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load .env variables
    dotenv().ok();

    let host = std::env::var("HOST").expect("Missing `HOST` env variable");
    let port = std::env::var("PORT").expect("Missing `PORT` env variable");
    let key = std::env::var("API_KEY").expect("Missing `API_KEY` env variable");
    let key = Key::new(key);

    // configure logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // database connection pool
    let db_pool = establish_connection(DatabaseKind::Example);

    // run pending migrations
    let connection = db_pool.get().unwrap();
    run_pending_migrations(&connection)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("Starting GraphQL server at http://{}:{}", host, port);

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(schema.clone())
            .data(key.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"]),
            ) // allow all cross origin requests
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            }))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
