# Base
FROM rust:1.73

ARG APP_ENV

WORKDIR /app

# Install cargo-watch
RUN cargo install cargo-watch

ENV APP_ENVIRONMENT ${APP_ENV}

COPY ../.. .
