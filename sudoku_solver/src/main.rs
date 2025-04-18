mod json_handler;
mod solvers;
mod sudoku;
mod validator;
use anyhow::Result;
use solvers::{
    backtracking::BacktrackingSolver,
    solver::{Solver, SolverKind, solve_with_strategy},
};

use sudoku::Sudoku;
use validator::Validator;

pub fn solve_sudoku_boards_from_json(file_path: &str) -> Result<Vec<Sudoku>> {
    let contents = json_handler::read_file(file_path)?;

    if contents.trim().is_empty() {
        return Err(anyhow::anyhow!("File '{}' is empty", file_path));
    }

    let sudoku_boards: Vec<Sudoku> = json_handler::parse_sudoku_boards(&contents)
        .map_err(|err| anyhow::anyhow!("Failed to parse Sudoku boards: {}", err))?;

    let mut valid_boards: Vec<Sudoku> = Vec::new();

    for (i, mut sudoku) in sudoku_boards.into_iter().enumerate() {
        if Validator::is_valid_board(&sudoku) {
            match BacktrackingSolver::solve(&mut sudoku) {
                Ok(_) => {
                    valid_boards.push(sudoku);
                    println!("Sudoku #{} solved successfully.", i + 1);
                }
                Err(_) => {
                    eprintln!(
                        "Error: Sudoku #{} is valid but unsolvable, skipping.",
                        i + 1
                    );
                }
            }
        } else {
            eprintln!("Error: Sudoku #{} is invalid, skipping.", i + 1);
        }
    }

    if valid_boards.is_empty() {
        return Err(anyhow::anyhow!("No valid Sudoku boards found"));
    }

    Ok(valid_boards)
}

fn solve_sudoku_from_string<BS: Solver>(input: &str, expected_output: &str) {
    let mut sudoku = Sudoku::from_string(input).expect("Invalid Sudoku");

    BS::solve(&mut sudoku).expect("Failed to solve Sudoku");
    let solved_sudoku_str = sudoku.to_string();
    assert_eq!(
        solved_sudoku_str, expected_output,
        "The solution doesn't fit!"
    );
    println!("Sudoku solved correctly!");
}

fn solve_with_strategy_test(kind: SolverKind, input: &str, expected_output: &str) {
    let mut sudoku = Sudoku::from_string(input).expect("Invalid Sudoku");

    solve_with_strategy(&mut sudoku, kind).expect("Failed to solve Sudoku");

    let solved_sudoku_str = sudoku.to_string();
    assert_eq!(
        solved_sudoku_str, expected_output,
        "The solution doesn't fit!"
    );

    println!("Sudoku solved correctly!");
}

fn main() {
    let input = "050000024904005000876240000010002080300000750409017200000900000247000000000600032";
    let expected_output =
        "153786924924135678876249315715362489362498751489517263638921547247853196591674832";
    solve_sudoku_from_string::<BacktrackingSolver>(input, expected_output);
    solve_with_strategy_test(SolverKind::Backtracking, input, expected_output);
    solve_with_strategy_test(SolverKind::Heuristic, input, expected_output);
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn expect_sudoku_solution<BS: Solver>(input: &str, expected_output: &str) {
        let mut sudoku = Sudoku::from_string(input).expect("Invalid Sudoku");

        BS::solve(&mut sudoku).expect("Failed to solve sudoku");
        let solved_sudoku_str = sudoku.to_string();
        assert_eq!(
            solved_sudoku_str, expected_output,
            "The solution doesn't fit!"
        );
        println!("Sudoku solved correctly!");
    }

    #[test]
    fn test_strategy_easy_sudoku_from_string_backtracking_solution() {
        let input =
            "050000024904005000876240000010002080300000750409017200000900000247000000000600032";
        let expected_output =
            "153786924924135678876249315715362489362498751489517263638921547247853196591674832";
        solve_with_strategy_test(SolverKind::Backtracking, input, expected_output);
    }

    #[test]
    fn test_strategy_easy_sudoku_from_string_heuristic_solution() {
        let input =
            "050000024904005000876240000010002080300000750409017200000900000247000000000600032";
        let expected_output =
            "153786924924135678876249315715362489362498751489517263638921547247853196591674832";
        solve_with_strategy_test(SolverKind::Heuristic, input, expected_output);
    }

    #[test]
    fn test_easy_sudoku_from_string_backtracking_solution() {
        let input =
            "050000024904005000876240000010002080300000750409017200000900000247000000000600032";
        let expected_output =
            "153786924924135678876249315715362489362498751489517263638921547247853196591674832";
        expect_sudoku_solution::<BacktrackingSolver>(input, expected_output);
    }

    #[test]
    fn test_medium_sudoku_from_string_backtracking_solution() {
        let input =
            "000000000603140500902500807520090614300000000001005209730800000009000006060010070";
        let expected_output =
            "475289163683147592912563847527398614396421758841675239734856921159732486268914375";

        expect_sudoku_solution::<BacktrackingSolver>(input, expected_output);
    }

    #[test]
    fn test_fiendish_sudoku_from_str_backtracking_solution() {
        let input =
            "000100597650009310000000004001003700060407000005800900030028000006000003070030001";
        let expected_output =
            "423186597657249318918375264891563742362497185745812936134728659286951473579634821";

        expect_sudoku_solution::<BacktrackingSolver>(input, expected_output);
    }

    #[test]
    fn test_solve_single_valid_board_from_json() {
        let path = "inputs/first.json";

        let result = solve_sudoku_boards_from_json(path);

        assert!(result.is_ok(), "Expected a valid solution, got error");

        let solved_boards = result.unwrap();
        assert_eq!(solved_boards.len(), 1, "Expected 1 solved board");
        println!("Solved Sudoku: \n{}", solved_boards[0]);
    }

    #[test]
    fn test_solve_multiple_boards_from_json() {
        let path = "inputs/multiple_boards.json";

        let result = solve_sudoku_boards_from_json(path);
        assert!(result.is_ok(), "Expected valid solutions, got error");

        let solved_boards = result.unwrap();
        assert!(
            solved_boards.len() > 1,
            "Expected more than one solved board"
        );
    }

    #[test]
    fn test_empty_json_file() {
        let path = "inputs/empty.json";

        let result = solve_sudoku_boards_from_json(path);
        assert!(result.is_err(), "Expected error for empty file");
    }

    #[test]
    fn test_invalid_json_format() {
        let path = "inputs/invalid.json";

        let result = solve_sudoku_boards_from_json(path);
        assert!(result.is_err(), "Expected error for invalid JSON format");
    }

    fn create_temp_file(contents: &str) -> std::io::Result<String> {
        let tmp_dir = std::env::temp_dir();
        let file_path = tmp_dir.join("test_sudoku.json");
        let mut file = std::fs::File::create(&file_path)?;
        file.write_all(contents.as_bytes())?;
        Ok(file_path.to_str().unwrap().to_string())
    }

    #[test]
    fn test_solve_sudoku_boards() {
        let contents = r#"[
                {"board": [[5, 3, 0, 0, 7, 0, 0, 0, 0], [6, 0, 0, 1, 9, 5, 0, 0, 0], [0, 9, 8, 0, 0, 0, 0, 6, 0], [8, 0, 0, 0, 6, 0, 0, 0, 3], [4, 0, 0, 8, 0, 3, 0, 0, 1], [7, 0, 0, 0, 2, 0, 0, 0, 6], [0, 6, 0, 0, 0, 0, 2, 8, 0], [0, 0, 0, 4, 1, 9, 0, 0, 5], [0, 0, 0, 0, 8, 0, 0, 7, 9]]}
            ]"#;
        let file_path = create_temp_file(contents).expect("Failed to create temp file");

        let result = solve_sudoku_boards_from_json(&file_path);
        assert!(
            result.is_ok(),
            "Expected to solve Sudoku boards successfully"
        );
    }

    #[test]
    fn test_file_with_invalid_sudoku_boards() {
        let contents = r#"[
                {"board": [[5, 3, 5, 6, 7, 0, 0, 0, 0], [6, 0, 0, 1, 9, 5, 0, 0, 0], [0, 9, 8, 0, 0, 0, 0, 6, 0], [8, 0, 0, 0, 6, 0, 0, 0, 3], [4, 0, 0, 8, 0, 3, 0, 0, 1], [7, 0, 0, 0, 2, 0, 0, 0, 6], [0, 6, 0, 0, 0, 0, 2, 8, 0], [0, 0, 0, 4, 1, 9, 0, 0, 5], [0, 0, 0, 0, 8, 0, 0, 7, 9]]}
            ]"#;
        let file_path = create_temp_file(contents).expect("Failed to create temp file");

        let result = solve_sudoku_boards_from_json(&file_path);
        assert!(
            result.is_err(),
            "Expected error due to invalid Sudoku board"
        );
    }

    #[test]
    fn test_file_with_no_sudoku_boards() {
        let contents = r#"[]"#;
        let file_path = create_temp_file(contents).expect("Failed to create temp file");

        let result = solve_sudoku_boards_from_json(&file_path);
        assert!(result.is_err(), "Expected error due to no Sudoku boards");
    }
}
