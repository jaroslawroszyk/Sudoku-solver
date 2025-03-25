pub struct Validator;

impl Validator {
    pub fn is_valid(board: &Vec<Vec<u8>>, row: usize, col: usize, value: u8) -> bool {
        if board[row].contains(&value) {
            return false;
        }

        for i in 0..9 {
            if board[i][col] == value {
                return false;
            }
        }
        /*
           for i in 0..9 {
            if board[row][i] == value || board[i][col] == value {
                return false;
            }
        }
        /
        */

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

        assert!(Validator::is_valid(&board, 0, 2, 4));
        assert!(Validator::is_valid(&board, 4, 4, 5));
        assert!(!Validator::is_valid(&board, 0, 1, 3));
    }
}
