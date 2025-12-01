use std::collections::HashMap;
use std::sync::LazyLock;

static DIRECTION_MAP: LazyLock<HashMap<char, i32>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert('R', 1);
    map.insert('L', -1);
    map
});

pub fn solve(input: &str) {
    let (unlocks, global_unlocks) = rotate(input);

    println!("Unlocks: {}", unlocks);
    println!("Global Unlocks: {}", global_unlocks);
}

fn rotate(input: &str) -> (i32, i32) {

    let lines: Vec<&str> = input.trim().lines().collect();
    let mut spinner_position = 50;
    let mut unlocks = 0;
    let mut global_unlocks = 0;

    for line in lines {
        if !validate_action(line) {
            panic!("Invalid action: {}", line);
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        let mult = DIRECTION_MAP.get(&direction).unwrap();

        let initial_position = spinner_position;

        global_unlocks += distance / 100;

        if (spinner_position > 0 && spinner_position + (distance * mult) % 100 < 0) || spinner_position + (distance * mult) % 100 > 100 {
            global_unlocks += 1;
        }

        spinner_position = (spinner_position + (distance * mult)).rem_euclid(100);

        if spinner_position == 0 {
            unlocks += 1;
            if initial_position == 0 {
                global_unlocks -= 1;
            }
        }
    }

    global_unlocks += unlocks;

    (unlocks, global_unlocks)
}

fn validate_action(action: &str) -> bool {
    if action.len() < 2 {
        return false;
    }

    let direction = action.chars().next().unwrap();
    if !DIRECTION_MAP.contains_key(&direction) {
        return false;
    }

    let distance_str = &action[1..];
    match distance_str.parse::<i32>() {
        Ok(distance) => distance > 0,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let test_cases = [
            ("", (0, 0)),
            ("R1", (0, 0)),
            ("R50", (1, 1)),
            ("L100", (0, 1)),
            ("R100", (0, 1)),
            ("L51", (0, 1)),
            ("R51", (0, 1)),
            ("R50\nL100", (2, 2)),
            ("R50\nL100\nR123", (2, 3)),
            ("R49\nR2", (0, 1)),
            ("L49\nL2", (0, 1)),
            ("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82", (3, 6)),
        ];

        for (input, expected) in test_cases {
            assert_eq!(rotate(input), expected);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid action: 0")]
    fn test_rotate_invalid_input() {
        let input = "0";
        rotate(input);
    }

    #[test]
    fn test_validate_action() {
        assert_eq!(validate_action("R1"), true);
        assert_eq!(validate_action("L1"), true);
        assert_eq!(validate_action("R9999"), true);
        assert_eq!(validate_action("0"), false);
        assert_eq!(validate_action("1"), false);
        assert_eq!(validate_action("-1"), false);
        assert_eq!(validate_action("A1"), false);
        assert_eq!(validate_action("R0"), false);
        assert_eq!(validate_action("L-1"), false);
    }
}
