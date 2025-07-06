# syntax=docker/dockerfile:1

# --- Build stage ---
FROM rust:1.88-alpine AS builder
RUN apk add --no-cache build-base protobuf-dev
WORKDIR /usr/src/app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY proto ./proto
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && rm -rf src

# Build actual sources
COPY . .
RUN cargo build --release

# --- Runtime stage ---
FROM alpine:3.18
RUN apk add --no-cache libgcc
COPY --from=builder /usr/src/app/target/release/cpu-stresser /usr/local/bin/cpu-stresser
EXPOSE 20051
ENV RUST_LOG=info
CMD ["cpu-stresser", "server"]
