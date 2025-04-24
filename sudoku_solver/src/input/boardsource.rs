use anyhow::Result;

use crate::sudoku::Sudoku;

use super::{csv_handler::CsvHandler, json_handler::JsonHandler};

// pub trait BoardSource {
//     fn load_from_file(path: &str) -> Result<Vec<Sudoku>>;
// }

// TODO:
pub enum FileFormat {
    Json,
    Csv,
    // Xml,
}

pub fn load_boards_by_format(format: FileFormat, path: &str) -> Result<Vec<Sudoku>> {
    match format {
        FileFormat::Json => JsonHandler::load_from_file(path),
        FileFormat::Csv => CsvHandler::load_from_file(path),
        // FileFormat::Xml => XmlHandler::load_from_file(path),
    }
}

pub fn detect_format_from_path(path: &str) -> Option<FileFormat> {
    if path.ends_with(".json") {
        Some(FileFormat::Json)
    } else if path.ends_with(".csv") {
        Some(FileFormat::Csv)
    }
    // else if path.ends_with(".xml") {
    // Some(FileFormat::Xml)
    // }
    else {
        None
    }
}

//
// }
/*
if let Some(format) = detect_format_from_path(file_path) {
    let boards = load_boards_by_format(format, file_path)?;
*/
