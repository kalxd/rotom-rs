FROM sample as builder

COPY .sqlx .sqlx
COPY src src

RUN cargo build --release

FROM 192.168.31.10:5000/rust:1.88.0

COPY --from=builder /opt/app/target/release/rotom-rs /opt/app/rotom-rs

EXPOSE 3000

CMD ["./rotom-rs"]
