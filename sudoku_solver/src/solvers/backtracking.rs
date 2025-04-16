use super::solver::Solver;
use crate::{sudoku::Sudoku, validator::Validator};
use anyhow::Result;

pub struct BacktrackingSolver;

impl Solver for BacktrackingSolver {
    fn solve(board: &mut Sudoku) -> Result<(), anyhow::Error> {
        solve_with_validator(board, &|board, row, col, value| {
            Validator::is_valid(board, row, col, value)
        })
    }
}

fn solve_with_validator<F>(board: &mut Sudoku, is_valid: &F) -> Result<()>
where
    F: Fn(&Sudoku, usize, usize, u8) -> bool,
{
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                let mut solved = false;
                for value in 1..=9 {
                    if is_valid(board, row, col, value) {
                        board[row][col] = value;
                        if solve_with_validator(board, is_valid).is_ok() {
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
