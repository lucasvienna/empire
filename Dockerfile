FROM lukemathwalker/cargo-chef:latest-rust-1.92.0 AS chef
WORKDIR /app
RUN apt-get update && apt-get install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same, all layers should be cached.
COPY . .
RUN cargo build --release --bin empire

FROM debian:bookworm-slim AS runtime

ENV APP_ENVIRONMENT=production

WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates libpq-dev \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# We need the configuration files at runtime!
COPY config config
COPY --from=builder /app/target/release/empire empire

ENTRYPOINT ["./empire"]
