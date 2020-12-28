# Overview

[![CI][ci_badge]][ci]
[![codecov][codecov_badge]][codecov]

This is an example project that puts together Rust, Actix, Juniper and Diesel.

I've started with only very little knowledge of Rust. The learning curve was pretty steep but well worth the effort in the end. I'm sharing this example project in the hope that it will save someone time and prevent frustration.

## Main features

- GraphQL Server & Playground (Actix + Juniper)
- Queries using both POST and GET
- Database Access (Diesel + Postgres)
  - Pending migrations run automatically on web server startup
- Cors
- Authentication (API key in headers)

```
{
  "key": "123"
}
```

- Integration tests
- cargo-make support
- Docker Compose
- Github Actions workflows

  - CI: format, check, clippy, tests, code coverage
  - Security audit

# Setup

## Rust & Cargo

Install `rust` and `cargo` via `rustup` (https://rustup.rs/). The stable version is OK.

## cargo-make

```
cargo install cargo-make
```

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
psql -U postgres
CREATE DATABASE rust_graphql_example;
CREATE DATABASE rust_graphql_example_test;
```

# Run locally

Access to a postgres instance is required.

```
cargo make run
```

or

```
cargo make watch
```

Open http://localhost:8080/playground.

# Run Integration tests

```
cargo make test
```

# Run in Docker

```
docker volume create --name=rust-graphql-example-storage
docker-compose up
```

Open http://localhost:8080/playground.

[ci_badge]: https://github.com/mihai-dinculescu/rust-graphql-actix-juniper-diesel-example/workflows/CI/badge.svg?branch=master
[ci]: https://github.com/mihai-dinculescu/rust-graphql-actix-juniper-diesel-example/actions
[codecov_badge]: https://codecov.io/gh/mihai-dinculescu/rust-graphql-actix-juniper-diesel-example/branch/master/graph/badge.svg
[codecov]: https://codecov.io/gh/mihai-dinculescu/rust-graphql-actix-juniper-diesel-example
