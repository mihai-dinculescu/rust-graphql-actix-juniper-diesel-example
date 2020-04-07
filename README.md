# Overview
This is an example project that puts together Rust, Actix, Juniper and Diesel.

I've started with only very little knowledge of Rust. The learning curve was pretty steep but well worth the effort in the end. I'm sharing this example project in the hope that it will save someone time and prevent frustration.

## Main features
- GraphQL Server & Playground (Actix + Juniper)
- Database Access (Diesel + Postgres)
  - Pending migrations run automatically on web server startup
- Authentication (API key in headers)
```
{
  "key": "123"
}
```
- Cors
- Docker Compose

# Setup
Install `rust` and `cargo` via `rustup` (https://rustup.rs/). The stable version is OK.

Diesel CLI
```
cargo install diesel_cli --no-default-features --features postgres
```

Optional: Cargo Watch (not required, but it speeds up development greatly)
```
cargo install cargo-watch
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

# Run in Docker
```
docker volume create --name=graphql-example-storage
docker-compose up
```

Open http://localhost:8080/playground.
