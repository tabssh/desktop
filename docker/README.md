# TabSSH Docker

All build and run operations happen inside Docker — never run `cargo` on the host.

## Quick Start

### X11

```sh
xhost +local:docker
docker compose -f docker/docker-compose.yml up
```

### Wayland

```sh
docker compose -f docker/docker-compose.yml \
  -e WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
  -v "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" \
  up
```

## Development

Mount the source tree and use live `cargo run`:

```sh
docker compose -f docker/docker-compose.dev.yml up
```

Source changes are reflected immediately on the next `cargo run` invocation. The Cargo registry and build cache are persisted in named volumes so incremental builds remain fast.

## Running Tests

```sh
docker compose -f docker/docker-compose.test.yml run --rm test
```

Coverage report (requires `cargo-tarpaulin`, included in `casjaysdev/rust:latest`):

```sh
docker compose -f docker/docker-compose.test.yml run --rm coverage
```

## Building the Production Image

```sh
docker build -f docker/Dockerfile -t tabssh:latest .
```

Multi-arch (requires `docker buildx`):

```sh
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f docker/Dockerfile \
  -t tabssh:latest \
  --push .
```

## Display Forwarding Reference

### X11

```sh
xhost +local:docker
docker run --rm \
  -e DISPLAY="$DISPLAY" \
  -v /tmp/.X11-unix:/tmp/.X11-unix \
  tabssh:latest
```

### Wayland

```sh
docker run --rm \
  -e WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
  -e XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" \
  -v "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" \
  tabssh:latest
```

## SSH Keys

The compose file mounts `~/.ssh` read-only into `/home/tabssh/.ssh` so existing host keys are available without copying them into the image.

## Persistent Data

Named volumes `tabssh-data` and `tabssh-config` persist the application database and configuration across container restarts.
