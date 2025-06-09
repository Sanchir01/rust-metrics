FROM rust:1.87-slim as builder
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

COPY . .

RUN cargo build --release

EXPOSE 50051

CMD ["make","workspace"]


