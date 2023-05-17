FROM rust:1.66-slim-buster as builder

RUN apt-get update
RUN apt-get install pkg-config -y
# For optional postgres support
RUN apt-get install libssl-dev -y
RUN apt-get install libpq-dev -y

# Create a new empty project
RUN USER=root cargo new --bin coredb-gcp-issue
WORKDIR /coredb-gcp-issue

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rapid.toml ./rapid.toml
COPY ./public ./public

RUN cargo build --release
RUN rm -rf ./src

COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/coredb-gcp-issue*
RUN cargo install --path .

# Set the entry point for the container
CMD ["./target/release/coredb-gcp-issue"]
