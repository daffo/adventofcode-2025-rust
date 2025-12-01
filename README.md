# Advent of Code 2025 - Rust Solutions

My solutions to [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

This is a learning project where I'm using Advent of Code to learn Rust while on vacation.

## Structure

This project is organized as a Cargo workspace, with each day's challenge in its own package:

```
.
├── day01/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── day02/
├── ...
└── Cargo.toml
```

## Running Solutions

To run a specific day's solution:

```bash
cargo run --bin day01
```

To run all solutions:

```bash
cargo build --release
```

## Development

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Adding a New Day

Create a new day package:

```bash
cargo new --bin dayXX
```

Then add it to the workspace members in the root `Cargo.toml`:

```toml
[workspace]
members = [
    "day01",
    "dayXX",
]
```

## Progress

- [x] Day 01:

## License

This project is for personal learning purposes.
