mod sudoku;
mod validator;

use sudoku::Sudoku;

#[allow(dead_code)]
fn solve_sudoku_from_string(input: &str, expected_output: &str) {
    let mut sudoku = Sudoku::from_string(input).expect("Invalid sudoku");
    assert!(sudoku.solve(), "Sudoku not solved!");
    let solved_sudoku = sudoku.to_string();
    assert_eq!(solved_sudoku, expected_output, "The solution doesn't fit!");
    println!("Sudoku solved correctly!");
}

#[allow(dead_code)]
fn test_sudoku_str_1_solution() {
    let input = "050000024904005000876240000010002080300000750409017200000900000247000000000600032";
    let expected_output =
        "153786924924135678876249315715362489362498751489517263638921547247853196591674832";

    solve_sudoku_from_string(input, expected_output);
}

fn test_with_board(board: Vec<Vec<u8>>) {
    let mut sudoku = match Sudoku::new(board) {
        Ok(sudoku) => sudoku,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("{}", sudoku);

    if sudoku.solve() {
        println!("Solved:");
        println!("{}", sudoku);
    } else {
        println!("No solution found");
    }
}

#[allow(dead_code)]
fn test_with_board_1() {
    let board = vec![
        vec![6, 0, 2, 1, 0, 5, 0, 8, 0],
        vec![9, 8, 0, 0, 6, 0, 0, 0, 4],
        vec![7, 0, 0, 0, 0, 0, 6, 0, 0],
        vec![4, 0, 0, 9, 7, 2, 0, 0, 0],
        vec![8, 0, 0, 5, 0, 0, 0, 9, 0],
        vec![0, 0, 5, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 2, 5],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 9, 4, 0, 0, 0],
    ];
    test_with_board(board);
}

#[allow(dead_code)]
fn test_with_board_2() {
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
    test_with_board(board);
}

#[allow(dead_code)]
fn test_from_single_board_in_json() {
    let mut sudoku_boards: Vec<Sudoku> =
        Sudoku::from_json_file("inputs/first.json").expect("JSON parsing error");

    for (i, sudoku) in sudoku_boards.iter_mut().enumerate() {
        if sudoku.solve() {
            println!("Sudoku {} solved:", i + 1);
            println!("{}", sudoku);
        } else {
            println!("Sudoku {}: No solution found", i + 1);
        }
    }
}

#[allow(dead_code)]
fn test_from_multiple_board_in_json() {
    let mut sudoku_boards: Vec<Sudoku> =
        Sudoku::from_json_file("inputs/multiple_boards.json").expect("JSON parsing error");

    for (i, sudoku) in sudoku_boards.iter_mut().enumerate() {
        if sudoku.solve() {
            println!("Sudoku {} solved:", i + 1);
            println!("{}", sudoku);
        } else {
            println!("Sudoku {}: No solution found", i + 1);
        }
    }
}

fn main() {
    match Sudoku::from_json_file("inputs/multiple_boards.json") {
        Ok(valid_boards) => {
            for (i, sudoku) in valid_boards.iter().enumerate() {
                println!("Solved Sudoku #{}:\n{}", i + 1, sudoku);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

// fn main() {
//     // test_from_single_board_in_json();
//     // test_from_multiple_board_in_json();

//     // test_sudoku_str_1_solution();
//     // test_with_board_1();
//     // test_with_board_2();
// }
