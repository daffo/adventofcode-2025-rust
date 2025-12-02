pub fn solve(input: &str) {
    let result = find_invalid_ids(input);

    println!("Invalid ids: {}", result);
}

fn find_invalid_ids(input: &str) -> i64 {
    let pairs: Vec<&str> = input.trim().split(',').collect();
    let mut acc = 0;

    for pair in pairs {
        let parts: Vec<&str> = pair.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();

        acc += compute_range(start, end);
    }

    acc
}

fn compute_range(start: i64, end: i64) -> i64 {
    let mut acc: i64 = 0;

    let start_digits = start.to_string().len();
    let end_digits = end.to_string().len();

    if start_digits % 2 != 0 && end_digits % 2 != 0 {
        return acc;
    }

    for num in start..=end {
        let num_str = num.to_string();
        let num_digits = num_str.len();

        if num_digits % 2 != 0 {
            continue;
        }

        let mid = num_digits / 2;
        let first_half = &num_str[..mid];
        let second_half = &num_str[mid..];

        if first_half == second_half {
            acc += num;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(find_invalid_ids("1-3"), 0);
        assert_eq!(find_invalid_ids("1-11"), 11);
        assert_eq!(
            find_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }

    #[test]
    fn test_compute_range() {
        assert_eq!(compute_range(1, 3), 0);
        assert_eq!(compute_range(1, 11), 11);
        assert_eq!(compute_range(22, 33), 55);
        assert_eq!(compute_range(50, 60), 55);
    }
}
