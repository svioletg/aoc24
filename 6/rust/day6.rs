use std::{collections::HashSet, fs};

use aoc::MxPointMethods;

#[path = "../../utils/aoc.rs"]
mod aoc;

fn main() {
    let _sample_in: String = fs::read_to_string("6/sample.txt").unwrap();
    let real_in: String = fs::read_to_string("6/input.txt").unwrap();

    solve_puzzle(real_in);
}

fn guard_walk_loop(start: aoc::MxPoint, mat: &aoc::Matrix<char>, obstacle: char) -> usize {
    let mut pos: aoc::MxPoint = start;
    let mut dir: aoc::MxPoint = aoc::MxPoint(-1, 0);
    let mut visited: HashSet<aoc::MxPoint> = HashSet::new();

    loop {
        match aoc::matget(mat, pos + dir) {
            ch if ch == Some(&obstacle) => dir = dir.turn_90deg(),
            Some(_) => { pos = pos + dir; visited.insert(pos); () },
            None => { dbg!(pos + dir); break }
        }
    }

    visited.len()
}

fn solve_puzzle(puzzle_input: String) {
    let matrix = aoc::string_to_matrix(&puzzle_input);
    // Find guard char
    let mut guard_pos: Option<(usize, usize)> = None;
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, column) in row.iter().enumerate() {
            if *column == '^' { guard_pos = Some((row_idx, col_idx)); }
        }
    }

    if guard_pos.is_none() { panic!("No guard character found in the input string."); }
    let guard_pos: (usize, usize) = guard_pos.unwrap();

    let visited: usize = guard_walk_loop(guard_pos.into(), &matrix, '#');
    println!("{}", visited);
}
