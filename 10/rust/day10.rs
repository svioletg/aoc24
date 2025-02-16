use std::{collections::HashMap, env, fs, process};

#[path = "../../utils/aoc.rs"]
mod aoc;

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

    println!("Day 10, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let trailmap: aoc::Matrix<u32> = aoc::matrix_map(&aoc::string_to_matrix(&puzzle_input), |i| i.to_digit(10).unwrap());
}

fn find_trailheads(mat: aoc::Matrix<u32>) -> Vec<aoc::MxPoint> {
    let mut trailheads: Vec<aoc::MxPoint> = vec![];
    for (ridx, row) in mat.iter().enumerate() {
        for (cidx, column) in row.iter().enumerate() {
            if *column == 0 { trailheads.push(aoc::MxPoint(ridx as isize, cidx as isize)); }
        }
    }

    trailheads
}

fn find_trails(mat: aoc::Matrix<u32>, heads: Vec<aoc::MxPoint>) {
    let mut trails: HashMap<aoc::MxPoint, Vec<Vec<(aoc::MxPoint, u32)>>> = HashMap::new();
    let mut branches: Vec<aoc::MxPoint> = vec![];
    for head_pt in heads.iter() {
        let mut pts_checked: Vec<aoc::MxPoint> = vec![];
        let mut path: Vec<(aoc::MxPoint, u32)> = vec![(*head_pt, 0)];

        branches.push(*head_pt);
        while branches.len() > 0 {
            let last_path_pt: &(aoc::MxPoint, u32) = path.last().unwrap();
            let this_branch: aoc::MxPoint = branches.pop().expect("Unexpectedly exhausted branches!");
            if pts_checked.contains(&this_branch) { continue }
            pts_checked.push(this_branch);

            if this_branch != *head_pt {
                match aoc::matget(&mat, this_branch) {
                    Some(level) => {
                        if (*level > last_path_pt.1) && (*level - last_path_pt.1 == 1) { path.push((this_branch, *level)) }
                        else { continue }
                    },
                    None => continue
                }
            }

            let adj: Vec<aoc::MxPoint> = aoc::points_adjacent(this_branch, false, false);
            branches.extend(adj);
        }
    }
}
