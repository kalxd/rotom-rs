FROM 192.168.31.10:5000/rotom-rs:base-0.0.6 AS builder

COPY .sqlx .sqlx
COPY src src
COPY sql sql

RUN touch src/main.rs
RUN cargo build --release

FROM m.daocloud.io/docker.io/library/debian:trixie-slim

WORKDIR /opt/app

COPY --from=builder /opt/app/target/release/rotom-rs /opt/app/rotom-rs
COPY config/default.toml config/default.toml

EXPOSE 3000

CMD ["./rotom-rs"]
