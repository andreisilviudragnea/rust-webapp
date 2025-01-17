FROM rust:1.84 AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR app
COPY --from=builder /app/target/release/rust-webapp /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rust-webapp"]
