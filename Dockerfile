FROM rust:bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin modupdate

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/modupdate /usr/local/bin
RUN apt update && apt upgrade
RUN apt install -y libssl3 ca-certificates curl

HEALTHCHECK CMD curl --fail http://localhost:8000/health || exit 1

ENTRYPOINT ["/usr/local/bin/modupdate"]
