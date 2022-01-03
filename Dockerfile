FROM rust:1.57.0
RUN apt update && apt install mingw-w64 -y
RUN rustup target add x86_64-pc-windows-gnu