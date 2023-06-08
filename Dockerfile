FROM rust:1.66-slim-buster as build

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

RUN apt-get update && apt-get install -y lsb-release wget

# Installs the postgres APT repository
# https://wiki.postgresql.org/wiki/Apt
RUN apt-get update && apt-get install -y \
        curl ca-certificates gnupg lsb-release \
        && rm -rf /var/lib/apt/lists/*
RUN curl https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor | tee /etc/apt/trusted.gpg.d/apt.postgresql.org.gpg > /dev/null
RUN echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list

# Install postgres tools
RUN apt-get update && apt-get install -y postgresql-client libssl-dev pkg-config libpq-dev

RUN cargo build --release


# Run the binary
CMD ["./target/release/coredb-gcp-issue"]
