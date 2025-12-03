pub fn solve(input: &str) {
    for num_digits in [2, 12] {
        let result = find_best_batteries(input, num_digits);
        println!("Best battery sum ({}): {}", num_digits, result);
    }
}

fn find_best_batteries(input: &str, num_digits: usize) -> i64 {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut battery_sum: i64 = 0;

    for line in lines {
        battery_sum += process_row(line, num_digits);
    }

    battery_sum
}

fn process_row(line: &str, num_digits: usize) -> i64 {
    let mut best_combo: Vec<u32> = vec![0; num_digits];
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len() {
        let digit = chars[i].to_digit(10).unwrap();

        // When approaching the edge, start checking from later positions
        let start_j = if i >= chars.len() - (num_digits - 1) {
            i - (chars.len() - num_digits)
        } else {
            0
        };

        for j in start_j..num_digits {
            if digit > best_combo[j] {
                best_combo[j] = digit;
                // Reset right part of the acc
                for k in j+1..num_digits {
                    best_combo[k] = 0;
                }
                break;
            }
        }
    }

    let mut result: i64 = 0;
    for digit in &best_combo {
        result = result * 10 + (*digit as i64);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best_batteries() {
        assert_eq!(find_best_batteries("", 2), 0);

        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(find_best_batteries(input, 2), 357);
        assert_eq!(find_best_batteries(input, 12), 3121910778619);
    }

    #[test]
    fn test_process_row() {
        assert_eq!(process_row("12345", 2), 45);
        assert_eq!(process_row("11111", 2), 11);
        assert_eq!(process_row("91119", 2), 99);
        assert_eq!(process_row("119911", 2), 99);

        assert_eq!(process_row("123321", 3), 332);
        assert_eq!(process_row("1233241", 3), 341);
        assert_eq!(process_row("123241", 3), 341);
    }
}
