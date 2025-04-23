use anyhow::Result;

use crate::sudoku::Sudoku;

pub trait BoardSource {
    fn load_from_file(path: &str) -> Result<Vec<Sudoku>>;
}

// TODO:
// pub enum FileFormat {
//     Json,
//     Csv,
//     Xml,
// }

// pub fn load_boards_by_format(format: FileFormat, path: &str) -> Result<Vec<Sudoku>> {
//     match format {
//         FileFormat::Json => JsonHandler::load_from_file(path),
//         FileFormat::Csv => CsvHandler::load_from_file(path),
//         FileFormat::Xml => XmlHandler::load_from_file(path),
//     }
// }
