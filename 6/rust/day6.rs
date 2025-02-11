use std::fs;

#[path = "../../utils/aoc.rs"]
mod aoc;

fn main() {
    let sample_in: String = fs::read_to_string("6/sample.txt").unwrap();
    let real_in: String = fs::read_to_string("6/input.txt").unwrap();

    solve_puzzle(sample_in);
}

fn solve_puzzle(puzzle_input: String) {
    let matrix: Vec<Vec<char>> = aoc::string_to_matrix(&puzzle_input);
    // Find guard char
    let mut guard_pos: (usize, usize);
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, column) in row.iter().enumerate() {
            if *column == '^' { guard_pos = (row_idx, col_idx); }
        }
    }
}
