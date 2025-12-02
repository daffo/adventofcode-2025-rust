pub fn solve(input: &str) {
    let (half_repeats, all_repeats) = find_invalid_ids(input);

    println!("Half repeats: {}", half_repeats);
    println!("All repeats: {}", all_repeats);
}

fn find_invalid_ids(input: &str) -> (i64, i64) {
    let pairs: Vec<&str> = input.trim().split(',').collect();
    let mut half_repeats = 0;
    let mut all_repeats = 0;

    for pair in pairs {
        let parts: Vec<&str> = pair.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();

        let (h_r, a_r) = compute_range(start, end);
        half_repeats += h_r;
        all_repeats += a_r;
    }

    (half_repeats, all_repeats)
}

fn compute_range(start: i64, end: i64) -> (i64, i64) {
    let mut half_repeats: i64 = 0;
    let mut all_repeats: i64 = 0;

    for num in start..=end {
        let num_str = num.to_string();
        let num_digits = num_str.len();

        for chunk_size in (1..=(num_digits/2)).rev() {
            if num_digits % chunk_size != 0 {
                continue;
            }

            let mut all_equal = true;
            let first_chunk = &num_str[..chunk_size];

            for i in (chunk_size..num_digits).step_by(chunk_size) {
                let chunk = &num_str[i..i + chunk_size];
                if chunk != first_chunk {
                    all_equal = false;
                    break;
                }
            }

            if all_equal {
                all_repeats += num;
                if num_digits % 2 == 0 && chunk_size == num_digits/2 {
                    half_repeats += num;
                }
                break;
            }
        }
    }

    (half_repeats, all_repeats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(find_invalid_ids("1-3"), (0, 0));
        assert_eq!(find_invalid_ids("1-11"), (11, 11));
        assert_eq!(
            find_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            (1227775554, 4174379265)
        );
    }

    #[test]
    fn test_compute_range() {
        assert_eq!(compute_range(1, 3), (0, 0));
        assert_eq!(compute_range(1, 11), (11, 11));
        assert_eq!(compute_range(22, 33), (55, 55));
        assert_eq!(compute_range(50, 60), (55, 55));
        assert_eq!(compute_range(111, 230), (0, 333));
        assert_eq!(compute_range(123123123, 123123124), (0, 123123123));
    }
}
