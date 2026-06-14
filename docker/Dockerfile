# =============================================================================
# Build Stage - Compile tabssh release binary
# =============================================================================
FROM casjaysdev/rust:latest AS builder

ARG TARGETARCH
ARG VERSION=dev
ARG BUILD_DATE
ARG VCS_REF

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets
COPY build.rs ./

RUN case "${TARGETARCH:-amd64}" in \
      amd64)  TARGET=x86_64-unknown-linux-musl ;; \
      arm64)  TARGET=aarch64-unknown-linux-musl ;; \
      *)      echo "Unsupported TARGETARCH: ${TARGETARCH}"; exit 1 ;; \
    esac && \
    cargo build --release --target "$TARGET" && \
    cp "target/$TARGET/release/tabssh" /tabssh

# =============================================================================
# Runtime Stage - Minimal image with static binary
# =============================================================================
FROM alpine:latest

ARG VERSION=dev
ARG BUILD_DATE
ARG VCS_REF
ARG LICENSE=MIT

RUN apk add --no-cache \
    bash \
    tini \
    libx11 \
    libxcb \
    wayland-libs-client \
    mesa-gl \
    ttf-dejavu

RUN mkdir -p /config /data /work
RUN adduser -D -h /home/tabssh tabssh

COPY --from=builder /tabssh /usr/local/bin/tabssh
COPY docker/rootfs/ /

RUN chmod +x /usr/local/bin/tabssh /usr/local/bin/entrypoint.sh

USER tabssh
WORKDIR /home/tabssh

ENTRYPOINT ["tini", "--", "/usr/local/bin/entrypoint.sh"]
CMD ["tabssh"]
