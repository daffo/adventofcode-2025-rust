pub fn solve(input: &str) {
    let result = rotate(input);

    println!("Unlocks: {}", result);
}

fn rotate(input: &str) -> i32 {
    use std::collections::HashMap;

    let direction_map: HashMap<char, i32> = [('R', 1), ('L', -1)].iter().cloned().collect();

    let lines: Vec<&str> = input.trim().lines().collect();
    let mut tracker = 50;
    let mut unlocks = 0;

    for line in lines {
        if !validate_action(line) {
            panic!("Invalid action: {}", line);
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        let mult = direction_map.get(&direction).unwrap();

        tracker = (tracker + (distance * mult)) % 100;

        if tracker == 0 {
            unlocks += 1;
        }
    }

    unlocks
}

fn validate_action(action: &str) -> bool {
    use std::collections::HashMap;

    let direction_map: HashMap<char, i32> = [('R', 1), ('L', -1)].iter().cloned().collect();

    if action.len() < 2 {
        return false;
    }

    let direction = action.chars().next().unwrap();
    if !direction_map.contains_key(&direction) {
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
            ("", 0),
            ("R1", 0),
            ("R50", 1),
            ("R50\nL100", 2),
            ("R50\nL100\nR123", 2),
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
