FROM rust:1.52-slim-buster as builder
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated \
    pkg-config python3 libgpgme-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app/
COPY Cargo.toml Cargo.toml
RUN mkdir src/ && echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN cargo build --release --verbose
RUN rm -f target/release/deps/gpg-tui*
COPY . .
RUN cargo build --locked --release --verbose
RUN mkdir -p build-out && cp target/release/gpg-tui build-out/

FROM debian:buster-slim as runner
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated \
    libgpgme-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /root/
COPY --from=builder /app/build-out/gpg-tui .
CMD ["./gpg-tui"]
