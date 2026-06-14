#!/usr/bin/env bash
set -euo pipefail

# Set up display forwarding if DISPLAY or WAYLAND_DISPLAY is set
if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then
    echo "Warning: Neither DISPLAY nor WAYLAND_DISPLAY is set. GUI may not work." >&2
    echo "Pass -e DISPLAY=:0 and mount /tmp/.X11-unix for X11, or" >&2
    echo "mount \$XDG_RUNTIME_DIR and set WAYLAND_DISPLAY for Wayland." >&2
fi

exec "$@"
