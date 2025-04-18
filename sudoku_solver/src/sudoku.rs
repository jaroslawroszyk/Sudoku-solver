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
    fn test_valid_board() {
        let valid_board = vec![
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

        let result = Sudoku::new(valid_board.clone());
        assert!(result.is_ok(), "Expected valid board");
        assert_eq!(result.unwrap().board, valid_board);
    }

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
    fn test_from_string_valid() {
        let input =
            "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
        let sudoku = Sudoku::from_string(input);
        assert!(sudoku.is_ok());
        assert_eq!(sudoku.unwrap().to_string(), input);
    }

    #[test]
    fn test_from_string_invalid_length() {
        let input = "123"; // too short
        let result = Sudoku::from_string(input);
        assert!(result.is_err(), "Expected error for short input");
    }

    #[test]
    fn test_from_string_non_digit() {
        let input =
            "53007000060019500009800006080006000X400803001700020006060000280000419005000080079";
        let result = Sudoku::from_string(input);
        assert!(result.is_err(), "Expected error for non-digit characters");
    }

    #[test]
    fn test_to_string() {
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
        let sudoku = Sudoku::new(board.clone()).unwrap();
        let expected = board
            .into_iter()
            .flatten()
            .map(|n| n.to_string())
            .collect::<String>();
        assert_eq!(sudoku.to_string(), expected);
    }

    #[test]
    fn test_partial_eq() {
        let valid_board = vec![
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

        let sudoku1 = Sudoku::new(valid_board.clone()).unwrap();
        let sudoku2 = Sudoku::new(valid_board).unwrap();
        assert_eq!(sudoku1, sudoku2);
    }

    #[test]
    fn test_display_formatting() {
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
        let sudoku = Sudoku::new(board).unwrap();
        let display = format!("{}", sudoku);
        assert!(display.contains("+-----------------------+"));
        assert!(display.contains("| 5 3 . | . 7 . | . . . |"));
    }
}
