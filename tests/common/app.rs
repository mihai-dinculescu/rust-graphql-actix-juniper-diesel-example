use actix_http::error::Error;
use actix_http::Request;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App};

use diesel_migrations::{revert_latest_migration, run_pending_migrations};
use dotenv::dotenv;

use lib::db::{establish_connection, DatabaseKind};
use lib::handlers::graphql::graphql;
use lib::models::key::Key;
use lib::schema_graphql::create_schema;

pub async fn create_app(
) -> impl Service<Request = Request, Response = ServiceResponse, Error = Error> {
    // load .env variables
    dotenv().ok();

    let key = std::env::var("API_KEY").expect("Missing `API_KEY` env variable");
    let key = Key::new(key);

    // create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // database connection pool
    let db_pool = establish_connection(DatabaseKind::ExampleTest);

    // get connection
    let connection = db_pool.get().unwrap();

    // revert migration
    // this works currently only because there is a single migration
    // will need to figure out how to do this for multiple migrations
    revert_latest_migration(&connection).ok();

    // run pending migrations
    run_pending_migrations(&connection).expect("run pending migrations error");

    // http test server
    test::init_service(
        App::new()
            .data(db_pool.clone())
            .data(schema.clone())
            .data(key.clone())
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            ),
    )
    .await
}
