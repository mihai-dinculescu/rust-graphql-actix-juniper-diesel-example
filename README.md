# Overview
This is an example project that puts together Rust, Actix, Juniper and Diesel.

I've started with only very little knowledge of Rust. The learning curve was pretty steep but well worth the effort in the end. I'm sharing this example project in the hope that it will save someone time and prevent frustration.

## Main features
- GraphQL Server & Playground (Actix + Juniper)
- Queries using both POST and GET
- Database Access (Diesel + Postgres)
  - Pending migrations run automatically on web server startup
- Authentication (API key in headers)
```
{
  "key": "123"
}
```
- Cors
- Integration tests
- Docker Compose

# Setup
## Rust & Cargo
Install `rust` and `cargo` via `rustup` (https://rustup.rs/). The stable version is OK.

## Diesel CLI
```
cargo install diesel_cli --no-default-features --features postgres
```

Optional: Cargo Watch (not required, but it speeds up development greatly)
```
cargo install cargo-watch
```

## Databases
```
CREATE DATABASE rust_graphql_example;
CREATE DATABASE rust_graphql_example_test;
```

# Run locally
Access to a postgres instance is required.

```
cargo run
```
or
```
cargo watch -x run
```

Open http://localhost:8080/playground.

# Run Integration tests
```
cargo test
```

# Run in Docker
```
docker volume create --name=rust-graphql-example-storage
docker-compose up
```

Open http://localhost:8080/playground.
