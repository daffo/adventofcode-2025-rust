pub fn solve(input: &str) {
    let (matrix, operators) = parse_input(input);
    
    let part1_result = solve_part1(&matrix, &operators);
    println!("Part 1: {}", part1_result);

    let (matrix, operators) = parse_input_with_spaces(input);
    let part2_result = solve_part2(&matrix, &operators);
    println!("Part 2: {}", part2_result);
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let lines: Vec<&str> = input.lines().collect();
    
    // Last line contains operators
    let operators: Vec<char> = lines.last()
        .map(|line| line.split_whitespace().map(|s| s.chars().next().unwrap()).collect())
        .unwrap_or_default();
    
    // All lines except last are numbers
    let matrix: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    
    (matrix, operators)
}

fn parse_input_with_spaces(input: &str) -> (Vec<Vec<String>>, Vec<Option<char>>) {
    let lines: Vec<&str> = input.lines().collect();

    // Last line contains operators with spaces preserved
    let operators: Vec<Option<char>> = if let Some(last_line) = lines.last() {
        last_line.chars()
            .map(|c| {
                if c == ' ' {
                    None
                } else {
                    Some(c)
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    // All lines except last - store each character as string
    let matrix: Vec<Vec<String>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string())
                .collect()
        })
        .collect();

    (matrix, operators)
}

fn solve_part1(matrix: &[Vec<u64>], operators: &[char]) -> u64 {
    let mut results = Vec::new();
    
    let num_cols = operators.len();
    
    for col in 0..num_cols {
        let operator = operators[col];
        
        let mut column_result = if operator == '+' { 0 } else { 1 };
        
        for row in matrix {
            let value = row[col];
            if operator == '+' {
                column_result += value;
            } else if operator == '*' {
                column_result *= value;
            }
        }
        
        results.push(column_result);
    }
    
    results.iter().sum()
}

fn solve_part2(matrix: &[Vec<String>], operators: &[Option<char>]) -> u64 {
    // Find max row length
    let max_len = matrix.iter().map(|row| row.len()).max().unwrap_or(0);
    
    let mut results = Vec::new();
    let mut current_nums = Vec::new();
    
    // Process from RIGHT to LEFT
    for col in (0..max_len.max(operators.len())).rev() {
        // First collect vertical number at this position (top to bottom)
        let mut column_digits = String::new();
        for row in matrix {
            if col < row.len() {
                let ch = &row[col];
                if ch != " " {
                    column_digits.push_str(ch);
                }
            }
        }
        
        // If we have a number, add it to current group
        if !column_digits.is_empty() {
            if let Ok(num) = column_digits.parse::<u64>() {
                current_nums.push(num);
            }
        }
        
        // Then check if there's an operator at this position
        let op_at_pos = if col < operators.len() {
            operators[col]
        } else {
            None
        };
        
        // If there's an operator, apply it to accumulated numbers (including this column's number)
        if let Some(op) = op_at_pos {
            if !current_nums.is_empty() {
                let result = apply_operation(&current_nums, op);
                results.push(result);
                current_nums.clear();
            }
        }
    }
    
    // Sum all results
    results.iter().sum()
}

fn apply_operation(numbers: &[u64], operator: char) -> u64 {
    if operator == '+' {
        numbers.iter().sum()
    } else {
        numbers.iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1 2 3 4\n5 6 7 8\n10 20 30 40\n+ + * *";
        let (matrix, operators) = parse_input(input);
        
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0], vec![1, 2, 3, 4]);
        assert_eq!(matrix[1], vec![5, 6, 7, 8]);
        assert_eq!(matrix[2], vec![10, 20, 30, 40]);
        
        assert_eq!(operators.len(), 4);
        assert_eq!(operators, vec!['+', '+', '*', '*']);
    }
    
    #[test]
    fn test_solve_part1() {
        let input = "1 2 3 4\n5 6 7 8\n10 20 30 40\n+ + * *";
        let (matrix, operators) = parse_input(input);
        
        // Column 0: 1+5+10 = 16
        // Column 1: 2+6+20 = 28
        // Column 2: 3*7*30 = 630
        // Column 3: 4*8*40 = 1280
        // Sum: 16+28+630+1280 = 1954
        let result = solve_part1(&matrix, &operators);
        assert_eq!(result, 1954);
    }
    
    #[test]
    fn test_solve_part1_with_spaces() {
        let input = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +";
        let (matrix, operators) = parse_input(input);
        
        let part1 = solve_part1(&matrix, &operators);
        assert_eq!(part1, 4277556);
    }
    
    #[test]
    fn test_parse_input_with_spaces() {
        let input = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +";
        let (matrix, operators) = parse_input_with_spaces(input);
        
        // Check matrix preserves all characters including spaces
        println!("Matrix rows: {}", matrix.len());
        for (i, row) in matrix.iter().enumerate() {
            println!("Row {}: len={}, {:?}", i, row.len(), row);
        }
        
        // Check operators preserve positions
        println!("Operators: len={}, {:?}", operators.len(), operators);
        
        // First row: "123 328  51 64"
        assert_eq!(matrix[0].len(), 14); // Length of the line
        assert_eq!(matrix[0][0], "1");
        assert_eq!(matrix[0][1], "2");
        assert_eq!(matrix[0][2], "3");
        assert_eq!(matrix[0][3], " ");
        
        // Operators line: "*   +   *   +"
        assert_eq!(operators[0], Some('*'));
        assert_eq!(operators[1], None); // space
        assert_eq!(operators[2], None); // space
        assert_eq!(operators[3], None); // space
        assert_eq!(operators[4], Some('+'));
    }
    
    #[test]
    fn test_solve_part2() {
        let input = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +";
        let (matrix, operators) = parse_input_with_spaces(input);
        
        let result = solve_part2(&matrix, &operators);
        assert_eq!(result, 3263827);
    }
}
