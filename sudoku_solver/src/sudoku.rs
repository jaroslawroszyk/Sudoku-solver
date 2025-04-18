use crate::validator::Validator;
use anyhow::Result;
use serde::Deserialize;
use std::fmt;
use std::ops::{Deref, DerefMut};

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
}

// Implementing the Deref and DerefMut traits allows us to use Sudoku as a slice
// of Vec<Vec<u8>>. This is useful for passing the Sudoku instance to functions
// that expect a slice, such as the `solve` method in the Solver trait.
// This allows us to use Sudoku as a slice of Vec<Vec<u8>>. This is useful for passing the Sudoku instance to functions
impl Deref for Sudoku {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
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
}
