# Use the official Rust image as the base
FROM rust:1.84.1-bullseye

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    curl \
    unzip \
    git \
    wget \
    zip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust targets
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl \
    x86_64-pc-windows-gnu aarch64-pc-windows-msvc \
    x86_64-apple-darwin aarch64-apple-darwin

# Install Zig (required for cargo-zigbuild)
ENV ZIG_VERSION="0.13.0"
RUN wget https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && tar -xf zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && mv zig-linux-x86_64-${ZIG_VERSION} /opt/zig \
    && ln -s /opt/zig/zig /usr/local/bin/zig

# Verify Zig installation
RUN zig version

# Ensure Zig is available in PATH
ENV PATH="/opt/zig:$PATH"

# Install cargo-zigbuild
RUN cargo install cargo-zigbuild

# Set up working directory
WORKDIR /build

# Copy the project files
COPY . .

# Ensure dependencies are downloaded
RUN cargo fetch

# Build for multiple targets
RUN cargo zigbuild --release --target x86_64-unknown-linux-musl
RUN cargo zigbuild --release --target aarch64-unknown-linux-musl
RUN cargo zigbuild --release --target x86_64-pc-windows-gnu
RUN cargo zigbuild --release --target x86_64-apple-darwin
RUN cargo zigbuild --release --target aarch64-apple-darwin

CMD ["ls", "-lh"]
