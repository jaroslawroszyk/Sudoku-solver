pub struct Validator;

impl Validator {
    pub fn is_valid(board: &Vec<Vec<u8>>, row: usize, col: usize, value: u8) -> bool {
        for i in 0..9 {
            if board[row][i] == value || board[i][col] == value {
                return false;
            }
        }

        let box_row_start = (row / 3) * 3;
        let box_col_start = (col / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                if board[box_row_start + i][box_col_start + j] == value {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_valid_board(board: &Vec<Vec<u8>>) -> bool {
        for i in 0..9 {
            let mut seen_row = [false; 9];
            let mut seen_col = [false; 9];
            for j in 0..9 {
                if let Some(num_row) = board[i][j].checked_sub(1) {
                    if seen_row[num_row as usize] {
                        return false;
                    }
                    seen_row[num_row as usize] = true;
                }

                if let Some(num_col) = board[j][i].checked_sub(1) {
                    if seen_col[num_col as usize] {
                        return false;
                    }
                    seen_col[num_col as usize] = true;
                }
            }
        }

        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                let mut seen = [false; 9];
                for i in 0..3 {
                    for j in 0..3 {
                        if let Some(num) = board[box_row + i][box_col + j].checked_sub(1) {
                            if seen[num as usize] {
                                return false;
                            }
                            seen[num as usize] = true;
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::Validator;

    #[test]
    fn test_is_valid() {
        let board = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        assert!(Validator::is_valid_board(&board));
        assert!(Validator::is_valid(&board, 0, 2, 4));
        assert!(Validator::is_valid(&board, 4, 4, 5));
        assert!(!Validator::is_valid(&board, 0, 1, 3));
    }

    #[test]
    fn test_is_invalid() {
        let invalid_board = vec![
            vec![5, 3, 3, 0, 7, 0, 0, 0, 0], // Powtórzenie 3 w pierwszym wierszu
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert!(!Validator::is_valid_board(&invalid_board));
    }
}
