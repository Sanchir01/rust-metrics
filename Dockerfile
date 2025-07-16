FROM rust:1.88-slim AS builder
WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    make \
    libssl-dev \
    protobuf-compiler \
    build-essential \
    autoconf \
    libtool \
    && rustup component add rustfmt

COPY Cargo.toml Cargo.lock .env ./

RUN cargo build --release

COPY . .


EXPOSE 50051

CMD ["make","workspace"]


