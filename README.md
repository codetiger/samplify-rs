# samplify-rs

A lightweight and flexible sample data generator for Rust structs, powered by fake-rs.

## Features

- Simple derive macro for automatic sample data generation
- Configuration-driven data generation
- Support for various data types:
  - Names (first name, last name, full name)
  - Numbers (integers and floats with range support)
  - Dates (with range support)
  - Booleans (with probability)
  - UUIDs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
samplify-rs = "0.1.0"
```

## Quick Start

```rust
use samplify_rs::Sampleable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Sampleable)]
pub struct User {
    name: String,
    created_at: String,
    score: String,
    is_active: String,
}

fn main() -> Result<(), String> {
    let config_json = r#"
    {
        "name": {
            "type": "name",
            "subtype": "name"
        },
        "created_at": {
            "type": "date",
            "params": {
                "start": "2023-01-01T00:00:00Z",
                "end": "2024-12-31T23:59:59Z"
            }
        },
        "score": {
            "type": "number",
            "params": {
                "min": 0,
                "max": 100,
                "float": true,
                "decimals": 2
            }
        },
        "is_active": {
            "type": "boolean",
            "params": {
                "true_probability": 0.7
            }
        }
    }"#;

    let config: serde_json::Map<String, serde_json::Value> = 
        serde_json::from_str(config_json)?;

    let user = User::sample_with_config(&config)?;
    println!("{:#?}", user);

    Ok(())
}
```

## Generator Types and Configuration

### Name Generator
```json
{
    "type": "name",
    "subtype": "name"  // Optional: "first_name", "last_name", "name" (default)
}
```

### Number Generator
```json
{
    "type": "number",
    "params": {
        "min": 0,           // Optional: minimum value
        "max": 100,         // Optional: maximum value
        "float": true,      // Optional: generate float instead of integer
        "decimals": 2       // Optional: number of decimal places for floats
    }
}
```

### Date Generator
```json
{
    "type": "date",
    "params": {
        "start": "2023-01-01T00:00:00Z",  // Optional: start date
        "end": "2024-12-31T23:59:59Z"     // Optional: end date
    }
}
```

### Boolean Generator
```json
{
    "type": "boolean",
    "params": {
        "true_probability": 0.7  // Optional: probability of generating true (default: 0.5)
    }
}
```

### UUID Generator
```json
{
    "type": "uuid"
}
```

## Features Coming Soon

- Support for enum types
- Vector generation
- Nested object support
- More fake-rs generators
- Custom generator functions

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Credits

This project uses the following crates:
- [fake-rs](https://github.com/cksac/fake-rs) for data generation
- [chrono](https://github.com/chronotope/chrono) for date handling
- [uuid](https://github.com/uuid-rs/uuid) for UUID generation
- [serde](https://github.com/serde-rs/serde) for serialization