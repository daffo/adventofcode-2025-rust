pub fn solve(input: &str) {
    let result = multiply(input);
    println!("Result: {}", result);
}

fn multiply(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().lines().collect();
    let num1: i32 = lines[0].parse().expect("First line must be a number");
    let num2: i32 = lines[1].parse().expect("Second line must be a number");
    num1 * num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let input = "13\n17";
        assert_eq!(multiply(input), 221);
    }
}
