FROM rust:1.75.0-alpine3.19

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8001

WORKDIR /logging-service
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]