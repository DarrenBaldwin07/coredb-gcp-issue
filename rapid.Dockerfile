# Buid Dockerfile: docker build -t rapid-server -f ./rapid.Dockerfile .
# Run Dockerfile: docker run -p 8080:8080 rapid-server

FROM rust:1.66-slim-buster as build

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/coredb-gcp-issue"]
