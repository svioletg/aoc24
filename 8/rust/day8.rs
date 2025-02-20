use std::{collections::{HashMap, HashSet}, env, fs, process};
use aoc::{Matrix, MxPoint};

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("8/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("8/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day 8, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

fn find_antennas(matrix: &Matrix<char>) -> HashMap<MxPoint, char> {
    let mut antennas: HashMap<MxPoint, char> = HashMap::new();
    for (ridx, row) in matrix.rows.iter().enumerate() {
        for (cidx, column) in row.iter().enumerate() {
            if column.is_alphanumeric() { antennas.insert(MxPoint(ridx as isize, cidx as isize), *column); }
        }
    }

    antennas
}

fn find_antinodes(matrix: &Matrix<char>, antennas: &HashMap<MxPoint, char>, puzzle_part: i32) -> HashSet<MxPoint> {
    let mut antinodes: HashSet<MxPoint> = HashSet::new();
    for (pt1, freq1) in antennas.iter() {
        for (pt2, freq2) in antennas.iter() {
            if pt1 == pt2 { continue }
            if freq1 != freq2 { continue }

            if puzzle_part == 1 {
                let antinode_pt = *pt1 + (*pt1 - *pt2);
                if matrix.get(antinode_pt).is_some() { antinodes.insert(antinode_pt); }
            } else if puzzle_part == 2 {
                antinodes.extend(matrix.walk_pattern(*pt1, *pt1 - *pt2, true));
            }
        }
    }

    antinodes
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let matrix = Matrix::from_str(&puzzle_input);

    let antennas = find_antennas(&matrix);
    let antinodes = find_antinodes(&matrix, &antennas, puzzle_part);

    println!("{}", antinodes.len());
}
