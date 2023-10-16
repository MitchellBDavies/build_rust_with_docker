FROM rust:1.73.0 as test
WORKDIR /usr/src/myapp
COPY . .
RUN cargo test

FROM rust:1.73.0 as build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
CMD ["./target/release/build_rust_with_docker.exe"]