# Advent of Code 2025 - Rust Solutions

My solutions to [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

This is a learning project where I'm using Advent of Code to learn Rust while on vacation.

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

For faster execution (optimized build):

```bash
cargo run --release -- 1
```

## Development

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Adding a New Day

1. Create a new module file `src/days/dayXX.rs` using the template:

```rust
pub fn solve(input: &str) {
    let part1_result = part1(input);
    let part2_result = part2(input);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> String {
    // TODO: Implement part 1
    "Not implemented yet".to_string()
}

fn part2(input: &str) -> String {
    // TODO: Implement part 2
    "Not implemented yet".to_string()
}
```

2. Add the module to `src/days/mod.rs`:

```rust
pub mod dayXX;
```

3. Add the day to the match statement in `src/main.rs`:

```rust
match day {
    1 => days::day01::solve(&input),
    XX => days::dayXX::solve(&input),
    // ...
}
```

4. Create your input file at `inputs/XX.txt`

### Running Tests

```bash
cargo test
```

## License

This project is for personal learning purposes.
