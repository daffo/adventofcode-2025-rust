pub fn solve(input: &str) {
    let (matrix, operators) = parse_input(input);
    
    let part1_result = solve_part1(&matrix, &operators);
    println!("Part 1: {}", part1_result);
    
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

fn solve_part2(_matrix: &[Vec<u64>], _operators: &[char]) -> u64 {
    // TODO: Implement part 2
    0
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
}
