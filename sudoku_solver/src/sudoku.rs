use crate::validator::Validator;

pub struct Sudoku {
    board: Vec<Vec<u8>>,
}

impl Sudoku {
    pub fn new(board: Vec<Vec<u8>>) -> Self {
        Self { board }
    }

    pub fn print_board(&self) {
        println!("+---------------------+");

        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                println!("|-------+-------+-------|");
            }
            print!("| ");
            for (j, &num) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                if num == 0 {
                    print!(". ");
                } else {
                    print!("{} ", num);
                }
            }

            println!("|");
        }

        println!("+---------------------+");
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    for value in 1..10 {
                        if Validator::is_valid(&self.board, row, col, value) {
                            self.board[row][col] = value;
                            if self.solve() {
                                return true;
                            }
                            self.board[row][col] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut sudoku = Sudoku::new(vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        let sudoku_solved = Sudoku::new(vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ]);

        assert!(sudoku.solve());
        assert_eq!(sudoku.board, sudoku_solved.board);
    }
}
