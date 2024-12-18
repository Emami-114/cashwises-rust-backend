#FROM rust as planner
#WORKDIR /app
#RUN cargo install cargo-chef
#RUN apt-get update \
#    && apt-get install -y gcc default-libmysqlclient-dev pkg-config
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json
#
#
## stage 2
#FROM rust as cacher
#WORKDIR /app
#RUN cargo install cargo-chef
#
#COPY --from=planner /app/recipe.json recipe.json
#RUN apt-get update \
#    && apt-get install -y gcc default-libmysqlclient-dev pkg-config
#
#RUN cargo chef cook --release --recipe-path recipe.json
#
## stage 3
#
#FROM rust:1.77 as builder
#
#COPY . /app
#
#WORKDIR /app
#RUN apt-get update \
#    && apt-get install -y gcc default-libmysqlclient-dev pkg-config
#COPY --from=cacher /app/target target
#COPY --from=cacher /usr/local/cargo /usr/local/cargo
#
#RUN cargo build --release
#
#
#FROM scratch
#
#COPY --from=builder /app/target/release/cashwises-rust /app/cashwises-rust
##RUN #chmod +x /app/cashwises-rust
##WORKDIR /app
#CMD ["./app/cashwises-rust"]
#EXPOSE 8080



FROM messense/rust-musl-cross:x86_64-musl-amd64 as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /cashwises-rust

# Stage 1: Planner
FROM chef AS planner
# Copy source code from previous stage
COPY . .
COPY ./templates .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Builder
FROM chef AS builder
# Copy recipe from planner stage
COPY --from=planner /cashwises-rust/recipe.json recipe.json


# Install build dependencies
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y gcc default-libmysqlclient-dev pkg-config \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*



# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .

# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 3: Final
FROM scratch


COPY --from=builder /cashwises-rust/target/x86_64-unknown-linux-musl/release/cashwises-rust /cashwises-rust
COPY --from=builder /cashwises-rust/templates /templates

ENTRYPOINT ["./cashwises-rust"]

EXPOSE 8000
