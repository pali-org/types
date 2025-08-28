# Contributing to Pali Types

## Development Setup

```bash
# Clone the repository
git clone https://github.com/pali-org/types.git
cd types

# Run tests
cargo test
```

## Before Submitting PRs

Run the local CI checks to ensure your code meets quality standards:

```bash
./scripts/ci-check.sh
```

This runs the same checks as our CI pipeline:
- Code formatting (`cargo fmt`)
- Linting (`cargo clippy`)
- Tests
- Documentation build

## Code Standards

- **Zero warnings**: All clippy warnings must be fixed
- **Formatted code**: Use `cargo fmt` before committing
- **Tests**: Add tests for new functionality
- **Documentation**: Document public APIs with examples

## Library Design

This is a foundational types library used by:
- **[Terminal Client](https://github.com/pali-org/terminal)** - CLI/TUI interfaces
- **[Server](https://github.com/pali-org/server)** - Cloudflare Workers API

Changes should be:
- **Backward compatible** when possible
- **Well documented** with examples
- **Thoroughly tested** with edge cases