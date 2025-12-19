# TabSSH Desktop - Build Automation

.PHONY: build release release-devel test docker clean

# Configuration
PROJECT := tabssh
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
COMMIT := $(shell git rev-parse --short=8 HEAD 2>/dev/null || echo "unknown")
BUILD_DATE := $(shell date "+%m/%d/%Y at %H:%M:%S")
DOCKER_IMAGE := tabssh-builder
DOCKER_TAG := latest

# Docker run with display support and build env
DOCKER_RUN := docker run --rm \
	-v $(PWD):/workspace \
	-w /workspace \
	-e DISPLAY=$(DISPLAY) \
	-e TABSSH_BUILD_COMMIT=$(COMMIT) \
	-e TABSSH_BUILD_DATE="$(BUILD_DATE)" \
	-v /tmp/.X11-unix:/tmp/.X11-unix \
	-v $(HOME)/.Xauthority:/root/.Xauthority:ro \
	--network host \
	$(DOCKER_IMAGE):$(DOCKER_TAG)

# Build all platforms + host binary
build:
	@echo "=== Building $(PROJECT) v$(VERSION) ==="
	@mkdir -p binaries

	@# Build Docker image if needed
	@docker inspect $(DOCKER_IMAGE):$(DOCKER_TAG) >/dev/null 2>&1 || \
		docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) -f scripts/docker/Dockerfile .

	@# Build for host (native binary)
	@echo "Building $(PROJECT)..."
	@$(DOCKER_RUN) cargo build --release
	@cp target/release/$(PROJECT) binaries/$(PROJECT)
	@strip binaries/$(PROJECT)

	@# Build Linux amd64 (static)
	@echo "Building $(PROJECT)-linux-amd64..."
	@$(DOCKER_RUN) cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/$(PROJECT) binaries/$(PROJECT)-linux-amd64
	@strip binaries/$(PROJECT)-linux-amd64

	@# Generate checksums
	@cd binaries && sha256sum $(PROJECT)* > checksums.txt 2>/dev/null || true

	@echo ""
	@echo "=== Build complete ==="
	@ls -lh binaries/

# Release to GitHub (versioned - adds 'v' prefix)
release: build
	@echo "=== Releasing $(PROJECT) v$(VERSION) to GitHub ==="
	@mkdir -p releases/v$(VERSION)
	@cp binaries/$(PROJECT)-* releases/v$(VERSION)/
	@cp binaries/checksums.txt releases/v$(VERSION)/
	@cd releases/v$(VERSION) && sha256sum * > checksums.txt
	-gh release delete v$(VERSION) --yes 2>/dev/null || true
	-git tag -d v$(VERSION) 2>/dev/null || true
	-git push origin :refs/tags/v$(VERSION) 2>/dev/null || true
	gh release create v$(VERSION) \
		--title "$(PROJECT) v$(VERSION)" \
		--notes "**$(PROJECT) v$(VERSION)**\n\n- Commit: \`$(COMMIT)\`\n- Built: $(BUILD_DATE)" \
		releases/v$(VERSION)/*
	@echo "Released v$(VERSION)"

# Release devel branch (no 'v' prefix)
release-devel: build
	@echo "=== Releasing $(PROJECT) devel to GitHub ==="
	@mkdir -p releases/devel
	@cp binaries/$(PROJECT)-* releases/devel/
	@cp binaries/checksums.txt releases/devel/
	@cd releases/devel && sha256sum * > checksums.txt
	-gh release delete devel --yes 2>/dev/null || true
	-git tag -d devel 2>/dev/null || true
	-git push origin :refs/tags/devel 2>/dev/null || true
	git checkout -B devel 2>/dev/null || git checkout devel
	git add -A
	git commit -m "Release devel - $(COMMIT) - $(BUILD_DATE)" 2>/dev/null || echo "No changes"
	git push -u origin devel --force
	gh release create devel \
		--target devel \
		--title "devel" \
		--notes "**$(PROJECT) Development Build**\n\n- Commit: \`$(COMMIT)\`\n- Built: $(BUILD_DATE)\n- Branch: devel" \
		releases/devel/*
	@echo "Released devel"

# Run tests
test:
	@docker inspect $(DOCKER_IMAGE):$(DOCKER_TAG) >/dev/null 2>&1 || \
		docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) -f scripts/docker/Dockerfile .
	@$(DOCKER_RUN) cargo test

# Build and push Docker image to ghcr.io
docker:
	@echo "=== Building and pushing Docker image ==="
	docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) -f scripts/docker/Dockerfile .
	docker tag $(DOCKER_IMAGE):$(DOCKER_TAG) $(DOCKER_IMAGE):v$(VERSION)
	docker push $(DOCKER_IMAGE):$(DOCKER_TAG)
	docker push $(DOCKER_IMAGE):v$(VERSION)
	@echo "Pushed $(DOCKER_IMAGE):$(DOCKER_TAG) and $(DOCKER_IMAGE):v$(VERSION)"

# Clean build artifacts
clean:
	rm -rf target binaries releases
