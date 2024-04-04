FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef

WORKDIR /cashwises-rust

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /cashwises-rust/recipe.json recipe.json

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y gcc default-libmysqlclient-dev pkg-config \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM --platform=linux/amd64 alpine:latest

COPY --from=builder /cashwises-rust/target/x86_64-unknown-linux-musl/release/cashwises-rust /cashwises-rust
ENTRYPOINT ["./cashwises-rust"]
EXPOSE 8080


################
##### Builder
# FROM rust:1.77.0-slim as builder
# ENV SQLX_OFFLINE=true

# WORKDIR /usr/src

# # Create blank project
# RUN USER=root cargo new cashwises-rust

# # We want dependencies cached, so copy those first.
# COPY Cargo.toml Cargo.lock /usr/src/cashwises-rust/

# # Set the working directory
# WORKDIR /usr/src/cashwises-rust

# ## Install target platform (Cross-Compilation) --> Needed for Alpine
# RUN rustup target add x86_64-unknown-linux-musl

# # This is a dummy build to get the dependencies cached.
# RUN cargo build --target x86_64-unknown-linux-musl --release

# # Now copy in the rest of the sources
# COPY src /usr/src/cashwises-rust/src/

# ## Touch main.rs to prevent cached release build
# RUN touch /usr/src/cashwises-rust/src/main.rs

# # This is the actual application build.
# RUN cargo build --target x86_64-unknown-linux-musl --release

# ################
# ##### Runtime
# FROM alpine:3.19.0 AS runtime

# # Copy application binary from builder image
# COPY --from=builder /usr/src/cashwises-rust/target/x86_64-unknown-linux-musl/release/cashwises-rust /usr/local/bin

# EXPOSE 8800

# # Run the application
# CMD ["/usr/local/bin/medium-rust-dockerize"]