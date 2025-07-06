FROM 192.168.31.10:5000/rust:build-1.88.0

COPY Cargo.toml .
COPY Cargo.lock .
COPY drv drv

RUN mkdir -p src
RUN echo "fn main() {}" > src/main.rs

RUN cargo chef prepare --recipe-path recipe.json
RUN cargo build --release
RUN cargo chef cook --release --recipe-path recipe.json
