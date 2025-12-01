# Advent of Code 2025 - Rust Solutions

My solutions to [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Structure

This project is organized as a single Rust binary with a module for each day:

```
.
├── src/
│   ├── main.rs          # Main entrypoint
│   └── days/
│       ├── mod.rs
│       ├── day01.rs
│       ├── day02.rs
│       └── ...
├── inputs/
│   ├── 1.txt           # Input for day 1
│   ├── 2.txt           # Input for day 2
│   └── ...
└── Cargo.toml
```

## Running Solutions

To run a specific day's solution:

```bash
# Run day 1
cargo run -- 1

# Run day 2
cargo run -- 2
```

## Development

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Adding a New Day

1. Create a new module file `src/days/dayXX.rs` using the template from `day00.rs` or `day01.rs`

2. Add one line to the `register_days!` macro in `src/days/mod.rs`:

```rust
register_days! {
    0 => day00,
    1 => day01,
    XX => dayXX,  // Just add this line!
}
```

3. Create your input file at `inputs/XX.txt`

### Running Tests

```bash
cargo test
```

## License

This project is for personal learning purposes.
