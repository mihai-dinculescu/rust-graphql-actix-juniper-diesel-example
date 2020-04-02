FROM rust:slim-buster

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev

WORKDIR /app
COPY ["Cargo.toml", "Cargo.lock", "diesel.toml", "/app/"]
COPY migrations /app/migrations
COPY src /app/src

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]
