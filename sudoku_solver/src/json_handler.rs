use crate::sudoku::Sudoku;
use anyhow::{Result, anyhow};
use serde_json;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Result<String> {
    let mut file =
        File::open(file_path).map_err(|err| anyhow!("Failed to open the file: {}", err))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| anyhow!("Failed to read the file: {}", err))?;

    Ok(contents)
}

pub fn parse_sudoku_boards(contents: &str) -> Result<Vec<Sudoku>> {
    let sudoku_boards: Vec<Sudoku> =
        serde_json::from_str(contents).map_err(|err| anyhow!("Failed to parse JSON: {}", err))?;

    if sudoku_boards.is_empty() {
        return Err(anyhow!("No Sudoku boards found in the file"));
    }

    Ok(sudoku_boards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;

    #[test]
    fn test_read_file() {
        let content = r#"[{"board": [[5,3,0,0,7,0,0,0,0],[6,0,0,1,9,5,0,0,0],[0,9,8,0,0,0,0,6,0],[8,0,0,0,6,0,0,0,3],[4,0,0,8,0,3,0,0,1],[7,0,0,0,2,0,0,0,6],[0,6,0,0,0,0,2,8,0],[0,0,0,4,1,9,0,0,5],[0,0,0,0,8,0,0,7,9]]}]"#;
        let path = "test_sudoku.json";

        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = read_file(path);
        assert!(result.is_ok());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_parse_sudoku_boards() {
        let content = r#"[{"board": [[5,3,0,0,7,0,0,0,0],[6,0,0,1,9,5,0,0,0],[0,9,8,0,0,0,0,6,0],[8,0,0,0,6,0,0,0,3],[4,0,0,8,0,3,0,0,1],[7,0,0,0,2,0,0,0,6],[0,6,0,0,0,0,2,8,0],[0,0,0,4,1,9,0,0,5],[0,0,0,0,8,0,0,7,9]]}]"#;

        let result = parse_sudoku_boards(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_failed_to_parse_json() {
        let invalid_json = r#"
            {
                "board": [
                    [5, 3, 0, 0, 7, 0, 0, 0, 0],
                    [6, 0, 0, 1, 9, 5, 0, 0, 0],
                    [0, 9, 8, 0, 0, 0, 0, 6, 0],
                    [8, 0, 0, 0, 6, 0, 0, 0, 3],
                    [4, 0, 0, 8, 0, 3, 0, 0, 1],
                    [7, 0, 0, 0, 2, 0, 0, 0, 6],
                    [0, 6, 0, 0, 0, 0, 2, 8, 0],
                    [0, 0, 0, 4, 1, 9, 0, 0, 5],
                    [0, 0, 0, 0, 8, 0, 0, 7, 9]
                ]
            "#;

        let result = parse_sudoku_boards(invalid_json);

        if let Err(e) = result {
            assert!(
                e.to_string().contains("invalid type: map"),
                "Unexpected error: {}",
                e
            );
        } else {
            panic!("Expected error, but got Ok!");
        }
    }

    #[test]
    fn test_no_sudoku_boards_found() {
        let content = r#"[]"#;

        let path = "test_empty_json.json";
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_sudoku_boards(content);
        remove_file(path).unwrap();

        assert!(
            result.is_err(),
            "Expected error when no Sudoku boards are found"
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "No Sudoku boards found in the file"
        );
    }
}
