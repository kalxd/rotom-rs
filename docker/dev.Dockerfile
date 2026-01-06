FROM 192.168.31.10:5000/rotom-rs:base-0.0.5 AS builder

COPY .sqlx .sqlx
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
COPY drv drv
COPY sql sql

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /opt/app

COPY --from=builder /opt/app/target/release/rotom-rs /opt/app/rotom-rs
COPY config/default.toml config/default.toml

EXPOSE 3000

CMD ["./rotom-rs"]
