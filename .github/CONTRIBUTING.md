# Contributing to TabSSH Desktop

## Development Setup

1. Install Rust (1.75+):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone and build:
```bash
git clone https://github.com/tabssh/desktop
cd desktop
cargo build
```

3. Run tests:
```bash
cargo test
```

## Code Style

- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new features
- Document public APIs

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and checks
5. Submit PR with clear description

## Areas to Contribute

- Additional themes
- Platform-specific features
- Bug fixes
- Documentation
- Tests
- Performance improvements

## Questions?

Open an issue or discussion on GitHub.
