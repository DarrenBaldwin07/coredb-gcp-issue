FROM rust:1.66-slim-buster as build

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN apt-get update
RUN apt-get install pkg-config -y
RUN apt-get install libssl-dev -y
# Optional for postgres support
RUN apt-get install libpq-dev -y
RUN cargo build --release


# Run the binary
CMD ["./target/release/coredb-gcp-issue"]
