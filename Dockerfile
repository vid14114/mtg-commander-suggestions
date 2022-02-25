FROM rust:1.59.0

# cargo tools
RUN cargo install cargo-outdated cargo-tarpaulin cargo-udeps just

# Windows cross build
RUN apt update && apt install mingw-w64 -y
RUN rustup target add x86_64-pc-windows-gnu

# Build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build