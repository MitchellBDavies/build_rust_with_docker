# build_rust_with_docker
Hello! This repository is just a small personal playground to expirement primarily with Docker & Github Actions, along with a small amount of Rust. 


Run this locally at your own risk with the following commands:
docker build --target=runtime -t runtime-container .
docker run --rm -p 8080:8080 --name rust-server runtime-container
