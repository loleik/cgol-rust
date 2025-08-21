FROM rust:latest

WORKDIR /usr/src/cgol_rust_docker

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/cgol-rust"]