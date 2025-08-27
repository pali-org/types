# Pali Types

[![Crates.io](https://img.shields.io/crates/v/pali-types.svg)](https://crates.io/crates/pali-types)
[![Documentation](https://docs.rs/pali-types/badge.svg)](https://docs.rs/pali-types)

Shared data types for the Pali todo management system.

This crate contains all the common data structures used by both the Pali server and CLI client, ensuring consistency and type safety across the entire system.

## Features

- **Todo Management** - Core todo item structure and CRUD request types
- **API Key Management** - Authentication and authorization types
- **Consistent API Responses** - Standard response wrapper format
- **Serde Support** - Full serialization/deserialization support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pali-types = "0.1.0"
```

## Examples

### Creating a todo

```rust
use pali_types::{CreateTodoRequest, priority};

// Simple todo
let simple = CreateTodoRequest::new("Buy groceries");

// Todo with fluent builder pattern
let detailed = CreateTodoRequest::new("Important task")
    .with_description("This needs to be done urgently")
    .with_high_priority()
    .with_due_date(1640995200); // Unix timestamp

// Using priority constants
let request = CreateTodoRequest {
    title: "Buy groceries".to_string(),
    description: Some("Milk, bread, eggs".to_string()),
    priority: Some(priority::MEDIUM),
    due_date: None,
};
```

### Working with API keys

```rust
use pali_types::{KeyType, CreateApiKeyRequest};

let request = CreateApiKeyRequest {
    client_name: "My Todo App".to_string(),
    key_type: KeyType::Client,
};
```

### API responses

```rust
use pali_types::ApiResponse;

let response: ApiResponse<Vec<Todo>> = ApiResponse::success(todos);
let error_response: ApiResponse<()> = ApiResponse::error("Not found".to_string());
```

## License

MIT