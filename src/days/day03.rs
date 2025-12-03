pub fn solve(input: &str) {
    let result = find_best_batteries(input);
    println!("Best battery sum: {}", result);
}

fn find_best_batteries(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut battery_sum = 0;

    for line in lines {
        battery_sum += process_row(line);
    }

    battery_sum
}

fn process_row(line: &str) -> i32 {
    let mut best_combo = [0, 0];
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len()-1 {
        let digit = chars[i].to_digit(10).unwrap() as i32;

        if digit > best_combo[0] {
            best_combo[0] = digit;
            best_combo[1] = 0;
        } else if digit > best_combo[1] {
            best_combo[1] = digit;
        }
    }

    let last_digit = chars[chars.len()-1].to_digit(10).unwrap() as i32;
    if last_digit > best_combo[1] {
        best_combo[1] = last_digit;
    }

    best_combo[0] * 10 + best_combo[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best_batteries() {
        assert_eq!(find_best_batteries(""), 0);

        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(find_best_batteries(input), 357);
    }

    #[test]
    fn test_process_row() {
        assert_eq!(process_row("12345"), 45);
        assert_eq!(process_row("11111"), 11);
        assert_eq!(process_row("91119"), 99);
        assert_eq!(process_row("119911"), 99);
    }
}
