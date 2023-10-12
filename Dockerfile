FROM rust:1.73.0
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
CMD ["build_rust_with_docker"]