use anyhow::Result;
use super::solver::Solver;
use crate::{sudoku::Sudoku, validator::Validator};

pub struct BacktrackingSolver;

impl Solver for BacktrackingSolver {
    fn solve(board: &mut Sudoku) -> Result<(), anyhow::Error> {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == 0 {
                    let mut solved = false;
                    for value in 1..=9 {
                        if Validator::is_valid(board, row, col, value) {
                            board[row][col] = value;
                            if Self::solve(board).is_ok() {
                                solved = true;
                                break;
                            }
                            board[row][col] = 0;
                        }
                    }
                    if !solved {
                        return Err(anyhow::anyhow!("Unsolvable board"));
                    }
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
