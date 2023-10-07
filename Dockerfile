# Base
FROM lukemathwalker/cargo-chef:latest-rust-1.71 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Planner Stage
FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

# Builder Stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .

# Build our project
RUN cargo build --release --bin rfc8984-calendar

# Runtime Stage
FROM debian:bullseye-slim AS runtime
ARG APP_ENV
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rfc8984-calendar app

COPY configuration configuration
ENV APP_ENVIRONMENT ${APP_ENV}

ENTRYPOINT ["./app"]
