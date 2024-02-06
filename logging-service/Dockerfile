FROM rust:1.75 as builder

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build our application
RUN rm ./target/release/deps/app*
RUN cargo build --release

# The final base
FROM debian:buster-slim

COPY --from=builder /app/target/release/app .
ENTRYPOINT ["./app"]
