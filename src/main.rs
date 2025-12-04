mod days;

use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day_number>", args[0]);
        eprintln!("Example: {} 1", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Day must be a number");

    let input_path = format!("inputs/{}.txt", day);

    if !Path::new(&input_path).exists() {
        eprintln!("Input file not found: {}", input_path);
        eprintln!("Please create the input file first.");
        std::process::exit(1);
    }

    let input = fs::read_to_string(&input_path)
        .expect("Failed to read input file");

    println!("Running Day {}...\n", day);

    let start = Instant::now();
    days::solve_day(day, &input);
    let duration = start.elapsed();

    println!("\n⏱️  Completed in {:?}", duration);
}
