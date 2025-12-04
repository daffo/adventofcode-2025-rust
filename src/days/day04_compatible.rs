pub fn solve(input: &str) {
    let mut matrix = parse_to_matrix(input);
    println!("Matrix size: {}x{}", matrix.len(), matrix[0].len());

    let part1_result = find_accessible_rolls(&mut matrix);
    println!("Accessible rolls: {}", part1_result);

    let mut part2_total = part1_result;
    loop {
        remove_marked_papers(&mut matrix);
        let removed = find_accessible_rolls(&mut matrix);
        if removed > 0 {
            part2_total += removed;
        } else {
            break;
        }
    }
    println!("Total accessible rolls: {}", part2_total);
}

fn parse_to_matrix(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '@' => 1,
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect()
        })
        .collect()
}

fn find_accessible_rolls(matrix: &mut Vec<Vec<i8>>) -> i64 {
    let mut accessible_rolls = 0;
    let mut sliding_window = vec![vec![0u8; 3]; 3];
    let mut current_rolls;

    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 0..rows {
        // Reset sliding window at the start of each row (virtual -1 column position)
        current_rolls = reset_sliding_window(&mut sliding_window, matrix, row, rows);

        for col in 0..cols {
            let roll_delta = advance_sliding_window(&mut sliding_window, matrix, row, col, rows, cols);
            current_rolls += roll_delta;

            // Check if center is a paper (1) with < 5 papers around it
            if sliding_window[1][1] == 1 && current_rolls < 5 {
                accessible_rolls += 1;
                // Mark as removed in the matrix
                matrix[row][col] = -1;
            }
        }
    }

    accessible_rolls
}

fn reset_sliding_window(
    sliding_window: &mut Vec<Vec<u8>>,
    matrix: &Vec<Vec<i8>>,
    row: usize,
    rows: usize,
) -> i32 {
    *sliding_window = vec![vec![0u8; 3]; 3];

    // Initialize rightmost column with absolute values from column 0 of the matrix
    let mut current_rolls = 0;
    for r in 0..3 {
        let matrix_row = row as i32 + r as i32 - 1;
        if matrix_row >= 0 && matrix_row < rows as i32 {
            let val = matrix[matrix_row as usize][0].abs() as u8;
            sliding_window[r][2] = val;
            current_rolls += val as i32;
        }
    }

    current_rolls
}

fn advance_sliding_window(
    sliding_window: &mut Vec<Vec<u8>>,
    matrix: &Vec<Vec<i8>>,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> i32 {
    let mut roll_delta = 0;

    for r in 0..3 {
        // Subtract roll from the left column
        roll_delta -= sliding_window[r][0] as i32;

        // Shift column
        sliding_window[r][0] = sliding_window[r][1];
        sliding_window[r][1] = sliding_window[r][2];

        // Fill the right column with absolute value from matrix at col+1
        let matrix_row = row as i32 + r as i32 - 1;
        let matrix_col = col + 1;
        if matrix_row >= 0 && matrix_row < rows as i32 && matrix_col < cols {
            let val = matrix[matrix_row as usize][matrix_col].abs() as u8;
            sliding_window[r][2] = val;
            roll_delta += val as i32;
        } else {
            sliding_window[r][2] = 0;
        }
    }

    roll_delta
}

fn remove_marked_papers(matrix: &mut Vec<Vec<i8>>) {
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == -1 {
                *cell = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_matrix() {
        let input = ".@.\n@.@\n.@@";
        let matrix = parse_to_matrix(input);
        assert_eq!(matrix, vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ]);
    }

    #[test]
    fn test_reset_sliding_window() {
        let matrix = vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ];

        // Dirty window to ensure full reset
        let mut sliding_window = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];

        let current_rolls = reset_sliding_window(&mut sliding_window, &matrix, 1, 3);

        // After reset at row 1: rightmost column should be [matrix[0][0], matrix[1][0], matrix[2][0]]
        assert_eq!(sliding_window, vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);

        // Sum of rightmost column: 0 + 1 + 0 = 1
        assert_eq!(current_rolls, 1);
    }

    #[test]
    fn test_advance_sliding_window() {
        let matrix = vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ];

        let mut sliding_window = vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ];

        let roll_delta = advance_sliding_window(&mut sliding_window, &matrix, 1, 2, 3, 3);

        // After advancing: left column removed, center->left, right->center, new right from matrix col 2
        assert_eq!(sliding_window, vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![1, 1, 0],
        ]);

        // Removed 0+1+0=1 from left, added 0+0+0=0 from right (out of bounds), delta = -1
        assert_eq!(roll_delta, -1);
    }

    #[test]
    fn test_find_accessible_rolls() {
        let mut matrix = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(find_accessible_rolls(&mut matrix), 2);

        let mut matrix = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];
        assert_eq!(find_accessible_rolls(&mut matrix), 4);
    }

    #[test]
    fn test_solve_10x10() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let mut matrix = parse_to_matrix(input);
        assert_eq!(find_accessible_rolls(&mut matrix), 13);
    }

    #[test]
    fn test_remove_marked_papers() {
        let mut matrix = vec![
            vec![0, 1, -1],
            vec![-1, 1, 0],
            vec![1, -1, 1],
        ];

        remove_marked_papers(&mut matrix);

        assert_eq!(matrix, vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![1, 0, 1],
        ]);
    }
}
