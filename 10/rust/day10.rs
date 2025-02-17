use std::{collections::HashSet, env, fs, process};

#[path = "../../utils/aoc.rs"]
mod aoc;
use aoc::{MxPoint, Matrix};

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("10/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("10/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    solve_puzzle(real_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let trailmap: Matrix<u32> = aoc::matrix_map(&aoc::string_to_matrix(&puzzle_input), |i| i.to_digit(10).unwrap());
    let heads: Vec<MxPoint> = find_trailheads(&trailmap);

    let mut scores: u32 = 0;
    let mut ratings: u32 = 0;

    for hpt in heads {
        let mut ends: HashSet<MxPoint> = HashSet::new();
        let result = score_trailhead(&trailmap, hpt, 0, &mut ends);
        ratings += result;
        scores += ends.len() as u32;
    }

    if puzzle_part == 1 {
        println!("{}", scores);
    } else {
        println!("{}", ratings);
    }
}

fn find_trailheads(mat: &Matrix<u32>) -> Vec<MxPoint> {
    let mut trailheads: Vec<MxPoint> = vec![];
    for (ridx, row) in mat.iter().enumerate() {
        for (cidx, column) in row.iter().enumerate() {
            if *column == 0 { trailheads.push(MxPoint(ridx as isize, cidx as isize)); }
        }
    }

    trailheads
}

fn score_trailhead(trailmap: &Matrix<u32>, pt: MxPoint, level: u32, ends: &mut HashSet<MxPoint>) -> u32 {
    let mut rating: u32 = 0;

    if level == 9 {
        ends.insert(pt);
        return 1
    }

    let adj = aoc::points_adjacent(pt, false, false);
    for adj_pt in adj {
        match aoc::matget(&trailmap, adj_pt) {
            Some(val) => {
                if !(*val > level && *val == level + 1) { continue }
                else { rating += score_trailhead(trailmap, adj_pt, *val, ends) }
            }
            None => continue
        }
    }

    rating
}
