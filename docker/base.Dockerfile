FROM 192.168.31.10:5000/rust:1.97.1

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir -p src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
