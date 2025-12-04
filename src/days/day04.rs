pub fn solve(input: &str) {
    let matrix = parse_to_matrix(input);
    println!("Matrix size: {}x{}", matrix.len(), matrix[0].len());

    let result = find_accessible_rolls(&matrix);
    println!("Accessible rolls: {}", result);
}

fn parse_to_matrix(input: &str) -> Vec<Vec<u8>> {
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

fn find_accessible_rolls(matrix: &[Vec<u8>]) -> i64 {
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

            if sliding_window[1][1] == 1 && current_rolls < 5 {
                accessible_rolls += 1;
            }
        }
    }

    accessible_rolls
}

fn reset_sliding_window(
    sliding_window: &mut Vec<Vec<u8>>,
    matrix: &[Vec<u8>],
    row: usize,
    rows: usize,
) -> i32 {
    *sliding_window = vec![vec![0u8; 3]; 3];

    // Initialize rightmost column with values from column 0 of the matrix
    let mut current_rolls = 0;
    for r in 0..3 {
        let matrix_row = row as i32 + r as i32 - 1;
        if matrix_row >= 0 && matrix_row < rows as i32 {
            sliding_window[r][2] = matrix[matrix_row as usize][0];
            current_rolls += sliding_window[r][2] as i32;
        }
    }

    current_rolls
}

fn advance_sliding_window(
    sliding_window: &mut Vec<Vec<u8>>,
    matrix: &[Vec<u8>],
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

        // Fill the right column with new value from matrix at col+1
        let matrix_row = row as i32 + r as i32 - 1;
        let matrix_col = col + 1;
        if matrix_row >= 0 && matrix_row < rows as i32 && matrix_col < cols {
            sliding_window[r][2] = matrix[matrix_row as usize][matrix_col];
            roll_delta += sliding_window[r][2] as i32;
        } else {
            sliding_window[r][2] = 0;
        }
    }

    roll_delta
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
        let matrix = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(find_accessible_rolls(&matrix), 2);

        let matrix = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];
        assert_eq!(find_accessible_rolls(&matrix), 4);
    }

    #[test]
    fn test_solve_10x10() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let matrix = parse_to_matrix(input);
        assert_eq!(find_accessible_rolls(&matrix), 13);
    }
}
