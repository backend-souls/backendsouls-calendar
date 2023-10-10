# Use the official Ubuntu base image
FROM ubuntu:latest
WORKDIR /app

# Update the package manager and install necessary dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust binaries to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install cargo-watch
RUN cargo install bunyan

COPY . .

