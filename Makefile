# TabSSH Desktop - Build Automation

.PHONY: build release test docker clean help

# Configuration
PROJECT := tabssh
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
COMMIT := $(shell git rev-parse --short=8 HEAD 2>/dev/null || echo "unknown")
BUILD_DATE := $(shell date "+%Y-%m-%d %H:%M:%S")
YYMM := $(shell date "+%y%m")
DOCKER_IMAGE := tabssh-builder
DOCKER_TAG := latest

# Docker run command
DOCKER_RUN := docker run --rm \
	-v $(PWD):/workspace \
	-w /workspace \
	-e TABSSH_BUILD_COMMIT=$(COMMIT) \
	-e TABSSH_BUILD_DATE="$(BUILD_DATE)" \
	$(DOCKER_IMAGE):$(DOCKER_TAG)

# Build binaries with Docker → outputs to ./binaries
build:
	@echo "=== Building $(PROJECT) v$(VERSION) ==="
	@echo "Commit: $(COMMIT)"
	@echo "Date: $(BUILD_DATE)"
	@echo ""
	@mkdir -p binaries

	@# Build Docker image if needed
	@docker inspect $(DOCKER_IMAGE):$(DOCKER_TAG) >/dev/null 2>&1 || \
		(echo "Building Docker image..." && \
		docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) -f docker/Dockerfile .)

	@# Build for host (native binary)
	@echo "Building $(PROJECT) (native)..."
	@$(DOCKER_RUN) cargo build --release
	@cp target/release/$(PROJECT) binaries/$(PROJECT)
	@strip binaries/$(PROJECT) 2>/dev/null || true

	@# Build Linux amd64 (static musl)
	@echo "Building $(PROJECT)-linux-amd64 (musl)..."
	@$(DOCKER_RUN) cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/$(PROJECT) binaries/$(PROJECT)-linux-amd64
	@strip binaries/$(PROJECT)-linux-amd64 2>/dev/null || true

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

	@# Build Docker image if needed
	@docker inspect $(DOCKER_IMAGE):$(DOCKER_TAG) >/dev/null 2>&1 || \
		(echo "Building Docker image..." && \
		docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) -f docker/Dockerfile .)

	@# Build for host (native binary)
	@echo "Building $(PROJECT) (native)..."
	@$(DOCKER_RUN) cargo build --release
	@cp target/release/$(PROJECT) releases/$(PROJECT)
	@strip releases/$(PROJECT) 2>/dev/null || true

	@# Build Linux amd64 (static musl)
	@echo "Building $(PROJECT)-linux-amd64 (musl)..."
	@$(DOCKER_RUN) cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/$(PROJECT) releases/$(PROJECT)-linux-amd64
	@strip releases/$(PROJECT)-linux-amd64 2>/dev/null || true

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
	@docker inspect $(DOCKER_IMAGE):$(DOCKER_TAG) >/dev/null 2>&1 || \
		docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) -f docker/Dockerfile .
	@$(DOCKER_RUN) cargo test

# Build Docker image with buildx (multi-arch: amd64, arm64)
docker:
	@echo "=== Building Docker image with buildx ==="
	@echo "Platforms: linux/amd64, linux/arm64"
	@echo "Tags: :latest :$(VERSION) :$(COMMIT) :$(YYMM)"
	@echo ""
	
	@# Ensure buildx builder exists
	@docker buildx inspect tabssh-builder >/dev/null 2>&1 || \
		docker buildx create --name tabssh-builder --use
	
	@# Build and push multi-arch image (cannot use --load with multi-platform)
	docker buildx build \
		--platform linux/amd64,linux/arm64 \
		--tag $(DOCKER_IMAGE):latest \
		--tag $(DOCKER_IMAGE):$(VERSION) \
		--tag $(DOCKER_IMAGE):$(COMMIT) \
		--tag $(DOCKER_IMAGE):$(YYMM) \
		--file docker/Dockerfile \
		--push \
		.
	
	@echo ""
	@echo "Built and pushed images:"
	@echo "  $(DOCKER_IMAGE):latest"
	@echo "  $(DOCKER_IMAGE):$(VERSION)"
	@echo "  $(DOCKER_IMAGE):$(COMMIT)"
	@echo "  $(DOCKER_IMAGE):$(YYMM)"
	@echo "Platforms: linux/amd64, linux/arm64"

# Build Docker image for local use (single platform)
docker-local:
	@echo "=== Building local Docker image ==="
	docker build \
		--tag $(DOCKER_IMAGE):latest \
		--tag $(DOCKER_IMAGE):$(VERSION) \
		--file docker/Dockerfile \
		.
	@echo "Built local image: $(DOCKER_IMAGE):latest"

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
	@echo "  make build     - Build binaries with Docker → ./binaries"
	@echo "  make release   - Release build with archive → ./releases"
	@echo "  make test      - Run tests in Docker"
	@echo "  make docker    - Build Docker image (buildx multi-arch: amd64, arm64)"
	@echo "  make clean     - Remove build artifacts"
	@echo "  make help      - Show this help"
	@echo ""
	@echo "Current version: $(VERSION)"
	@echo "Current commit:  $(COMMIT)"
