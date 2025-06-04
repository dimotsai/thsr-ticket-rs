FROM rust:alpine3.21 AS builder

WORKDIR /app

RUN apk add libressl-dev pkgconfig musl-dev

COPY src ./src
COPY Cargo.lock ./
COPY Cargo.toml ./

RUN cargo build -r

FROM alpine

COPY --from=builder /app/target/release/thsr /usr/local/bin

ENTRYPOINT [ "thsr" ]
