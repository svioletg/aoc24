use std::{collections::HashMap, env, fs, process};

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

    println!("Day 10, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let trailmap: Matrix<u32> = aoc::matrix_map(&aoc::string_to_matrix(&puzzle_input), |i| i.to_digit(10).unwrap());
}

fn find_trailheads(mat: Matrix<u32>) -> Vec<MxPoint> {
    let mut trailheads: Vec<MxPoint> = vec![];
    for (ridx, row) in mat.iter().enumerate() {
        for (cidx, column) in row.iter().enumerate() {
            if *column == 0 { trailheads.push(MxPoint(ridx as isize, cidx as isize)); }
        }
    }

    trailheads
}

fn get_trailhead_stats(mat: Matrix<u32>, heads: Vec<MxPoint>) -> (u32, u32) {
    let mut scores: HashMap<MxPoint, u32> = HashMap::new();
    let mut ratings: HashMap<MxPoint, u32> = HashMap::new();
    for head_pt in heads.iter() {
        scores.insert(*head_pt, 0);
        ratings.insert(*head_pt, 0);

        let mut pts_visited: Vec<MxPoint> = vec![];
        let mut path: Vec<(MxPoint, u32)> = vec![(*head_pt, 0)];

        let mut branches: Vec<(MxPoint, u32)> = vec![(*head_pt, 0)];
        while branches.len() > 0 {
            let this_branch = branches.pop().unwrap();
            pts_visited.push(this_branch.0);

            let last_path_item: &(MxPoint, u32) = path.last().unwrap();
            path.push(this_branch);
            let adj: Vec<(MxPoint, Option<&u32>)> = aoc::points_adjacent(this_branch.0, false, false)
                .iter().map(|&i| (i, aoc::matget(&mat, i))).collect();
            for (pt, level) in adj.iter() {
                if level.is_some_and(|&i| i == last_path_item.1 + 1) { branches.push((*pt, *level.unwrap())) }
            }
        }

        // branches.push(*head_pt);
        // while branches.len() > 0 {
        //     let last_path_pt: &(MxPoint, u32) = path.last().unwrap();
        //     let this_branch: MxPoint = branches.pop().expect("Unexpectedly exhausted branches!");
        //     if pts_checked.contains(&this_branch) { continue }
        //     pts_checked.push(this_branch);

        //     if this_branch != *head_pt {
        //         match aoc::matget(&mat, this_branch) {
        //             Some(level) => {
        //                 if (*level > last_path_pt.1) && (*level - last_path_pt.1 == 1) { path.push((this_branch, *level)) }
        //                 else { continue }
        //             },
        //             None => continue
        //         }
        //     }

        //     let adj: Vec<MxPoint> = aoc::points_adjacent(this_branch, false, false);
        //     branches.extend(adj);
        // }
    }

    (0, 0)
}
