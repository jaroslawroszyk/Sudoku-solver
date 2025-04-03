use crate::validator::Validator;
use anyhow::Result;
use serde::Deserialize;
use std::{fmt, io::Read};

#[derive(Debug, Deserialize)]
pub struct Sudoku {
    board: Vec<Vec<u8>>,
}

impl Sudoku {
    pub fn new(board: Vec<Vec<u8>>) -> Result<Self> {
        if !Validator::is_valid_board(&board) {
            return Err(anyhow::anyhow!("Invalid board"));
        }

        Ok(Self { board })
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    for value in 1..=9 {
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

    fn contains_non_digit(input: &str) -> bool {
        !input.chars().all(|c| c.is_digit(10))
    }

    pub fn from_string(input: &str) -> Result<Self> {
        const SUDOKU_BOARD: usize = 9 * 9;
        if input.len() != SUDOKU_BOARD || Self::contains_non_digit(input) {
            return Err(anyhow::anyhow!("Invalid input string"));
        }

        let board = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            .chunks(9)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<u8>>>();

        Self::new(board)
    }

    pub fn to_string(&self) -> String {
        self.board
            .iter()
            .flatten()
            .map(|&n| n.to_string())
            .collect()
    }

    fn open_file(file_path: &str) -> Result<String> {
        match std::fs::File::open(file_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => Ok(contents),
                    Err(err) => Err(anyhow::anyhow!("Failed to read the file: {}", err)),
                }
            }
            Err(err) => Err(anyhow::anyhow!("Failed to open the file: {}", err)),
        }
    }

    pub fn from_json_file(file_path: &str) -> Result<Vec<Sudoku>> {
        let contents = match Self::open_file(file_path) {
            Ok(contents) => contents,
            Err(err) => return Err(anyhow::anyhow!("Error reading the file: {}", err)),
        };

        let sudoku_boards: Vec<Sudoku> = match serde_json::from_str(&contents) {
            Ok(parsed) => parsed,
            Err(err) => return Err(anyhow::anyhow!("Failed to parse JSON: {}", err)),
        };

        if sudoku_boards.is_empty() {
            return Err(anyhow::anyhow!("No Sudoku boards found in the file"));
        }

        let mut valid_boards: Vec<Sudoku> = sudoku_boards
            .into_iter()
            .filter(|sudoku| Validator::is_valid_board(&sudoku.board))
            .collect();

        if valid_boards.is_empty() {
            return Err(anyhow::anyhow!("No valid Sudoku boards found"));
        }

        for sudoku in valid_boards.iter_mut() {
            if !sudoku.solve() {
                return Err(anyhow::anyhow!("Failed to solve a valid Sudoku board"));
            }
        }

        Ok(valid_boards)
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+-----------------------+")?;
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                writeln!(f, "|-------+-------+-------|")?;
            }
            write!(f, "| ")?;
            for (j, &num) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                if num == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", num)?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-----------------------+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_board() {
        let invalid_board_with_duplicates = vec![
            vec![5, 3, 5, 6, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let result = Sudoku::new(invalid_board_with_duplicates);
        assert!(result.is_err(), "Expected error for invalid board");
    }

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
        ])
        // .unwrap();
        .expect("Failed to create Sudoku from valid board");

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
        ])
        // .unwrap();
        .expect("Failed to create Sudoku from valid board");

        assert!(sudoku.solve());
        assert_eq!(sudoku.board, sudoku_solved.board);
    }

    #[test]
    fn test_empty_board() {
        let empty_board = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut sudoku = Sudoku::new(empty_board).expect("Failed to create empty Sudoku");
        assert!(sudoku.solve(), "Empty board should be solvable!");
    }
}
