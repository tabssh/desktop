# TabSSH Desktop - Build Automation

.PHONY: build release test check docker run-gui clean help

# Configuration
PROJECT := tabssh
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
COMMIT := $(shell git rev-parse --short=8 HEAD 2>/dev/null || echo "unknown")
BUILD_DATE := $(shell date "+%Y-%m-%d %H:%M:%S")
DOCKER_IMAGE := casjaysdev/rust:latest

# Docker run command
DOCKER_RUN := docker run --rm \
	-v $(PWD):/work \
	-w /work \
	-e TABSSH_BUILD_COMMIT=$(COMMIT) \
	-e TABSSH_BUILD_DATE="$(BUILD_DATE)" \
	$(DOCKER_IMAGE)

# Build binaries with Docker → outputs to ./binaries
build:
	@echo "=== Building $(PROJECT) v$(VERSION) ==="
	@echo "Commit: $(COMMIT)"
	@echo "Date: $(BUILD_DATE)"
	@echo ""
	@mkdir -p binaries

	@# Build Linux x86_64 (static musl)
	@echo "Building $(PROJECT)-linux-x86_64 (musl)..."
	@$(DOCKER_RUN) cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/$(PROJECT) binaries/$(PROJECT)-linux-x86_64
	@strip binaries/$(PROJECT)-linux-x86_64 2>/dev/null || true

	@# Generate checksums
	@echo "Generating checksums..."
	@cd binaries && sha256sum $(PROJECT)* > checksums.txt 2>/dev/null || true

	@echo ""
	@echo "=== Build complete ==="
	@echo "Binaries in ./binaries:"
	@ls -lh binaries/

# Release build → outputs to ./releases with archive and release.txt
release:
	@echo "=== Release Build $(PROJECT) v$(VERSION) ==="
	@echo "Commit: $(COMMIT)"
	@echo "Date: $(BUILD_DATE)"
	@echo ""
	@mkdir -p releases

	@# Build Linux x86_64 (static musl)
	@echo "Building $(PROJECT)-linux-x86_64 (musl)..."
	@$(DOCKER_RUN) cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/$(PROJECT) releases/$(PROJECT)-linux-x86_64
	@strip releases/$(PROJECT)-linux-x86_64 2>/dev/null || true

	@# Build Linux aarch64 (static musl)
	@echo "Building $(PROJECT)-linux-aarch64 (musl)..."
	@$(DOCKER_RUN) cargo build --release --target aarch64-unknown-linux-musl
	@cp target/aarch64-unknown-linux-musl/release/$(PROJECT) releases/$(PROJECT)-linux-aarch64
	@strip releases/$(PROJECT)-linux-aarch64 2>/dev/null || true

	@# Generate checksums
	@echo "Generating checksums..."
	@cd releases && sha256sum $(PROJECT)* > checksums.txt 2>/dev/null || true

	@# Write release.txt
	@echo "Writing release.txt..."
	@echo "$(VERSION)" > releases/release.txt
	@echo "Commit: $(COMMIT)" >> releases/release.txt
	@echo "Built: $(BUILD_DATE)" >> releases/release.txt

	@# Create source archive (exclude VCS files)
	@echo "Creating source archive..."
	@tar --exclude-vcs \
		--exclude='./target' \
		--exclude='./binaries' \
		--exclude='./releases' \
		-czf releases/$(PROJECT)-$(VERSION)-source.tar.gz \
		--transform="s,^\.,$(PROJECT)-$(VERSION)," \
		.

	@echo ""
	@echo "=== Release complete ==="
	@echo "Release files in ./releases:"
	@ls -lh releases/
	@echo ""
	@cat releases/release.txt

# Run tests in Docker
test:
	@echo "=== Running tests ==="
	@$(DOCKER_RUN) cargo test

# Format check and clippy in Docker
check:
	@echo "=== Running fmt + clippy ==="
	@$(DOCKER_RUN) cargo fmt --all --check
	@$(DOCKER_RUN) cargo clippy --workspace --all-targets --all-features -- -D warnings

# Build the runtime Docker image (not the build toolchain — use casjaysdev/rust:latest for that)
docker:
	@echo "=== Building runtime Docker image ==="
	@docker build -f docker/Dockerfile -t $(PROJECT):latest .
	@echo "Built runtime image: $(PROJECT):latest"

# Run the GUI locally with X11 forwarding
run-gui:
	@echo "=== Running $(PROJECT) with X11 forwarding ==="
	@docker run --rm \
		-v $(PWD):/work \
		-w /work \
		-e DISPLAY=$(DISPLAY) \
		-v /tmp/.X11-unix:/tmp/.X11-unix \
		$(DOCKER_IMAGE) \
		cargo run --release --target x86_64-unknown-linux-musl

# Clean build artifacts
clean:
	@echo "=== Cleaning build artifacts ==="
	rm -rf target binaries releases
	@echo "Cleaned: target/ binaries/ releases/"

# Help
help:
	@echo "TabSSH Desktop - Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build     - Build binary with Docker → ./binaries (x86_64)"
	@echo "  make release   - Release build for all arches → ./releases"
	@echo "  make test      - Run tests in Docker"
	@echo "  make check     - Run cargo fmt --check + clippy in Docker"
	@echo "  make docker    - Build runtime Docker image"
	@echo "  make run-gui   - Run GUI with X11 forwarding"
	@echo "  make clean     - Remove build artifacts"
	@echo "  make help      - Show this help"
	@echo ""
	@echo "Build image:     $(DOCKER_IMAGE)"
	@echo "Current version: $(VERSION)"
	@echo "Current commit:  $(COMMIT)"
