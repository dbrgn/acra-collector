FROM debian:jessie

# Install dependencies
RUN apt-get update && \
    apt-get install \
        ca-certificates \
        curl \
        gcc \
        libc6-dev \
        libssl-dev build-essential pkg-config \
        -y --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Rust vars
ENV RUST_ARCHIVE=rust-1.17.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

# Install Rust
RUN mkdir -p /rust
WORKDIR /rust
RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

# Build acra-collector
ADD . /source
RUN cd /source && cargo build --release
