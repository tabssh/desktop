# TabSSH Desktop - Rust Build & Runtime Environment
# Supports building and running GUI app with X11/Wayland

FROM rustlang/rust:nightly-bookworm

# Install build + runtime dependencies
RUN apt-get update && apt-get install -y \
    # Build tools
    build-essential \
    pkg-config \
    cmake \
    git \
    # OpenSSL
    libssl-dev \
    # GUI build dependencies
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libfontconfig1-dev \
    libgtk-3-dev \
    libatk1.0-dev \
    libgdk-pixbuf-2.0-dev \
    libpango1.0-dev \
    # X11 runtime
    libx11-6 \
    libxcursor1 \
    libxrandr2 \
    libxi6 \
    libgl1-mesa-glx \
    libgl1-mesa-dri \
    libegl1-mesa \
    # Wayland runtime
    libwayland-client0 \
    libwayland-egl1 \
    # Fonts
    fonts-dejavu-core \
    # Cross-compilation (musl static)
    musl-tools \
    musl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust targets
RUN rustup target add x86_64-unknown-linux-musl

# Environment
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH

# For musl static builds
ENV CC_x86_64_unknown_linux_musl=musl-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc

WORKDIR /workspace

CMD ["cargo", "build"]
