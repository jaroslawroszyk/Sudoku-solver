use super::solver::Solver;
use crate::{sudoku::Sudoku, validator::Validator};
use anyhow::Result;

pub struct Heuristic;

impl Solver for Heuristic {
    fn solve(board: &mut Sudoku) -> Result<(), anyhow::Error> {
        solve_with_heuristic(board, &|board, row, col, value| {
            Validator::is_valid(board, row, col, value)
        })
    }
}

fn solve_with_heuristic<F>(board: &mut Sudoku, is_valid: &F) -> Result<()>
where
    F: Fn(&Sudoku, usize, usize, u8) -> bool,
{
    let mut empty_cell = None;
    let mut min_choices = 10;

    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                let mut valid_choices = 0;
                for value in 1..=9 {
                    if is_valid(board, row, col, value) {
                        valid_choices += 1;
                    }
                }

                if valid_choices < min_choices {
                    min_choices = valid_choices;
                    empty_cell = Some((row, col));
                }
            }
        }
    }

    if let Some((row, col)) = empty_cell {
        for value in 1..=9 {
            if is_valid(board, row, col, value) {
                board[row][col] = value;
                if solve_with_heuristic(board, is_valid).is_ok() {
                    return Ok(());
                }
                board[row][col] = 1;
            }
        }
        return Err(anyhow::anyhow!("Unsolvable board"));
    }

    Ok(())
}
