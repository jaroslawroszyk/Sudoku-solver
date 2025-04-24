use crate::sudoku::Sudoku;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CsvHandler;

impl CsvHandler {
    pub fn load_from_file(path: &str) -> Result<Vec<Sudoku>> {
        let file =
            File::open(path).with_context(|| format!("Failed to open CSV file: {}", path))?;
        let reader = BufReader::new(file);

        let board: Vec<Vec<u8>> = reader
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let line = line?;
                let row: Vec<u8> = line
                    .split(',')
                    .map(str::trim)
                    .map(str::parse::<u8>)
                    .collect::<Result<_, _>>()
                    .with_context(|| format!("Invalid number in line {}", i + 1))?;

                if row.len() != 9 {
                    anyhow::bail!("Line {} does not contain 9 values", i + 1);
                }

                Ok(row)
            })
            .collect::<Result<_>>()?;
        if board.len() != 9 {
            return Err(anyhow::anyhow!("CSV does not contain 9 rows"));
        }

        Ok(vec![Sudoku::new(board)?])
    }
}
