#FROM messense/rust-musl-cross:x86_64-musl as chef
#ENV SQLX_OFFLINE=true
#RUN cargo install cargo-chef
#
#WORKDIR /cashwises-rust
#
#FROM chef AS planner
## Copy source code from previous stage
#COPY . .
## Generate info for caching dependencies
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM chef AS builder
#COPY --from=planner /cashwises-rust/recipe.json recipe.json
#
#RUN apt-get update \
#    && apt-get upgrade -y \
#    && apt-get install -y gcc default-libmysqlclient-dev pkg-config \
#    && apt-get install -y ca-certificates tzdata \
#    && rm -rf /var/lib/apt/lists/*
#
## Build & cache dependencies
#RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
## Copy source code from previous stage
#COPY . .
## Build application
#RUN cargo build --release --target x86_64-unknown-linux-musl
#
## Create a new stage with a minimal image
#FROM scratch
#COPY --from=builder /cashwises-rust/target/x86_64-unknown-linux-musl/release/cashwises-rust /cashwises-rust
#ENTRYPOINT ["./cashwises-rust"]
#EXPOSE 8080

# Builder Stage
FROM rust:1.75 as builder
ENV SQLX_OFFLINE=true

# Create a new Rust project
RUN USER=root cargo new --bin cashwises-rust
WORKDIR /cashwises-rust

# Copy and build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --locked
RUN rm src/*.rs

# Copy the source code and build the application
COPY . .
RUN cargo build --release --locked

# Production Stage
FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /cashwises-rust/target/release/cashwises-rust ${APP}/cashwises-rust

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./cashwises-rust"]
EXPOSE 8080