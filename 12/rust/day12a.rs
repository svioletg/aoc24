use std::{env, fs, process};

use aoc::{Matrix, MxPoint};

#[path = "../../utils/aoc.rs"]
mod aoc;

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("12/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("12/input.txt").unwrap();

    let puzzle_part: i32 = 1;
    // let args: Vec<String> = env::args().collect();
    // let puzzle_part: i32 = match args[1].as_str() {
    //     "a" => 1,
    //     "b" => 2,
    //     _ => {
    //         println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
    //         process::exit(1);
    //     }
    // };

    println!("Day 12, Part {}", puzzle_part);

    solve_puzzle(sample_in, puzzle_part);
}

struct GardenRegion {
    plant: char,
    plots: Vec<MxPoint>,
    perimeter: u32,
    area: u32
}

impl GardenRegion {
    fn find_perimeter(&self) -> u32 {
        let mut perimeter: u32 = 0;
        for pt in self.plots.iter() {
            let mut plot_sides: u32 = 4;
            for pt2 in self.plots.iter() {
                if pt == pt2 { continue }
                if (pt.0 == pt2.0) && ((pt.1 - pt2.1).abs() == 1) { plot_sides -= 1; }
                if (pt.1 == pt2.1) && ((pt.0 - pt2.0).abs() == 1) { plot_sides -= 1; }
            }
            perimeter += plot_sides;
        }

        perimeter
    }

    fn from_list(data: Vec<(MxPoint, char)>) -> GardenRegion {
        let mut inst = GardenRegion{plant: data[0].1, plots: data.iter().map(|i| i.0).collect(), perimeter: 0, area: 0};
        inst.area = inst.plots.len() as u32;
        inst.perimeter = inst.find_perimeter();

        inst
    }
}

fn find_regions(mat: &Matrix<char>) -> Vec<GardenRegion> {
    let mut pts_checked: Vec<MxPoint> = vec![];
    let mut regions: Vec<Vec<(MxPoint, char)>> = vec![];

    fn search_branches(pt: &MxPoint, mat: &Matrix<char>, plot: char, pts_checked: &mut Vec<MxPoint>) -> Vec<(MxPoint, char)> {
        if pts_checked.contains(pt) { return vec![]; }
        pts_checked.push(*pt);
        let mut region: Vec<(MxPoint, char)> = vec![(*pt, plot)];
        let adj_pts = aoc::points_adjacent(*pt, false, false);
        for branch_pt in adj_pts {
            region.extend(search_branches(&branch_pt, mat, plot, pts_checked));
        }

        region
    }

    for (ridx, row) in mat.iter().enumerate() {
        for (cidx, column) in row.iter().enumerate() {
            let pt = MxPoint(ridx as isize, cidx as isize);
            if pts_checked.contains(&pt) { continue }
            let search_result: Vec<(MxPoint, char)> = search_branches(&pt, mat, *column, &mut pts_checked);
            regions.push(search_result);
        }
    }

    regions.iter().map(|i| GardenRegion::from_list(i.clone())).collect()
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let garden: Matrix<char> = aoc::string_to_matrix(&puzzle_input);
    let garden_regions: Vec<GardenRegion> = find_regions(&garden);
    println!("{}", garden_regions.iter().map(|i| i.area * i.perimeter).sum::<u32>());
}
