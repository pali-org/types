# Pali Types

[![CI](https://github.com/pali-org/types/actions/workflows/ci.yml/badge.svg)](https://github.com/pali-org/types/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pali-types.svg)](https://crates.io/crates/pali-types)
[![Documentation](https://docs.rs/pali-types/badge.svg)](https://docs.rs/pali-types)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Shared data types for the Pali todo management system.

## Usage

```toml
[dependencies]
pali-types = "0.1.0"
```

```rust
use pali_types::{CreateTodoRequest, priority};

let todo = CreateTodoRequest::new("Buy groceries")
    .with_high_priority()
    .with_due_date(1640995200);
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

MIT