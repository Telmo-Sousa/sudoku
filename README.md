# sudoku 🦀

**Sudoku CLI game: Play it or brute force it!**

## About the Game

This is a simple command-line Sudoku game written in Rust. You can either play the game manually, attempting to solve a randomly generated Sudoku puzzle, or opt to use the brute-force solver to find the solution automatically.

## Features

- **Random Board Generation**: The game generates a random Sudoku board with 17 pre-filled cells, which is the minimum number required to create a solvable puzzle.
- **Play Mode**: You can enter numbers manually and try to solve the puzzle yourself.
- **Brute-force Solver**: If you're stuck or just want to see the solution, you can use the built-in brute-force solver to complete the board.
- **Input Validation**: The game ensures that player moves are valid according to Sudoku rules, preventing any incorrect entries.

## Requirements

- Rust programming language installed ([Install Rust](https://www.rust-lang.org/))

## How to play

1. Ensure Rust is installed on your system.
2. Clone the repository:
    ```bash
    git clone https://github.com/Telmo-Sousa/sudoku.git
    cd sudoku
    ```
3. Run the game:
    ```bash
    cargo run --release
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Important
Have fun!
