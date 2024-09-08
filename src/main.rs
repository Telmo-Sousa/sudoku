use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

const GRID_SIZE: usize = 9;

fn display_board(board: &[[u8; GRID_SIZE]; GRID_SIZE]) {
    for row in board {
        for &num in row {
            if num == 0 {
                print!(" . ");
            } else {
                print!(" {} ", num);
            }
        }
        println!();
    }
}

fn is_valid(board: &[[u8; GRID_SIZE]; GRID_SIZE], row: usize, col: usize, num: u8) -> bool {

    for i in 0..GRID_SIZE {
        if board[row][i] == num {
            return false;
        }
    }

    for i in 0..GRID_SIZE {
        if board[i][col] == num {
            return false;
        }
    }

    let start_row = row / 3 * 3;
    let start_col = col / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }

    true
}

fn solve_sudoku(board: &mut [[u8; GRID_SIZE]; GRID_SIZE]) -> bool {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if board[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[row][col] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn generate_random_sudoku(board: &mut [[u8; GRID_SIZE]; GRID_SIZE], num_filled: usize) {
    let mut rng = thread_rng();
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            positions.push((row, col));
        }
    }

    positions.shuffle(&mut rng);

    let mut count = 0;
    for &(row, col) in positions.iter() {
        if count >= num_filled {
            break;
        }

        let mut nums: Vec<u8> = (1..=9).collect();
        nums.shuffle(&mut rng);

        for &num in nums.iter() {
            if is_valid(board, row, col, num) {
                board[row][col] = num;
                count += 1;
                break;
            }
        }
    }
}

fn player_input(board: &mut [[u8; GRID_SIZE]; GRID_SIZE]) {
    let mut input = String::new();
    loop {
        println!("Enter your move (format: row col number), or 'done' to finish:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed = input.trim();
        if trimmed == "done" {
            break;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() == 3 {
            if let (Ok(row), Ok(col), Ok(num)) = (
                parts[0].parse::<usize>(),
                parts[1].parse::<usize>(),
                parts[2].parse::<u8>(),
            ) {
                if row >= 1 && row <= 9 && col >= 1 && col <= 9 && num >= 1 && num <= 9 {
                    let (row, col) = (row - 1, col - 1); // Convert to 0-indexed
                    if board[row][col] == 0 && is_valid(board, row, col, num) {
                        board[row][col] = num;
                        display_board(&board);
                    } else {
                        println!("Invalid move! Try again.");
                    }
                } else {
                    println!("Invalid input! Row and column should be between 1 and 9, and number should be between 1 and 9.");
                }
            } else {
                println!("Invalid input! Please enter numbers only.");
            }
        } else {
            println!("Invalid format! Use the format: row col number (e.g., 1 1 5).");
        }
        input.clear();
    }
}

fn main() {
    let mut board: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    println!("Generating random Sudoku board...");
    generate_random_sudoku(&mut board, 17);
    println!("Initial random Sudoku board:");
    println!("");
    display_board(&board);

    loop {
        println!("");
        println!("Find more games at https://tsousa.dev");
        println!("Type 'play' to play the game, or 'bruteforce' to solve:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "play" => {
                player_input(&mut board);
                println!("You have completed your moves. Checking if the solution is correct...");
                if solve_sudoku(&mut board) {
                    println!("Congratulations! The board is solvable.");
                    display_board(&board);
                } else {
                    println!("Oops! It seems like the board is unsolvable with your input.");
                }
                break;
            }
            "bruteforce" => {
                println!("Solving the Sudoku using brute force...\n");
                if solve_sudoku(&mut board) {
                    println!("Solved Sudoku board:");
                    display_board(&board);
                } else {
                    println!("No solution exists for the given Sudoku board.");
                }
                break;
            }
            _ => println!("Invalid input. Please type 'play' or 'bruteforce'."),
        }
    }
}
