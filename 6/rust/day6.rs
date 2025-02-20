use std::{collections::{HashMap, HashSet}, env, fs, process};
use aoc::MxPointMethods;

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("6/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("6/input.txt").unwrap();

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

fn guard_walk_loop(start: aoc::MxPoint, mat: &aoc::Matrix<char>, obst_char: char) -> (HashSet<aoc::MxPoint>, bool) {
    let mut pos: aoc::MxPoint = start;
    let mut dir: aoc::MxPoint = aoc::MxPoint(-1, 0);
    let mut spaces_visited: HashSet<aoc::MxPoint> = HashSet::from([start]);
    let mut obst_hit: HashMap<aoc::MxPoint, Vec<aoc::MxPoint>> = HashMap::new();
    let mut path_loops: bool = false;

    loop {
        match aoc::matget(mat, pos + dir) {
            ch if ch == Some(&obst_char) => {
                let this_obst = obst_hit.entry(pos + dir).or_insert(vec![]);
                if !this_obst.contains(&dir) { this_obst.push(dir); }
                else { path_loops = true; break }
                dir = dir.turn_90deg();
            },
            Some(_) => { pos = pos + dir; spaces_visited.insert(pos); () },
            None => break
        }
    }

    (spaces_visited, path_loops)
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let mut matrix = aoc::string_to_matrix(&puzzle_input);
    // Find guard char
    let mut guard_pos: Option<(usize, usize)> = None;
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, column) in row.iter().enumerate() {
            if *column == '^' { guard_pos = Some((row_idx, col_idx)); }
        }
    }

    println!("Matrix is {} by {}", matrix.len(), matrix[0].len());

    if guard_pos.is_none() { panic!("No guard character found in the input string."); }
    let guard_pos: (usize, usize) = guard_pos.unwrap();

    if puzzle_part == 1 {
        let visited = guard_walk_loop(guard_pos.into(), &matrix, '#').0;
        println!("{}", visited.len());
    } else if puzzle_part == 2 {
        let visited = guard_walk_loop(guard_pos.into(), &matrix, '#');
        let mut possible_obsts: usize = 0;

        for loc in visited.0 {
            if loc == guard_pos.into() { continue }

            let was: char = *aoc::matget(&matrix, loc).unwrap();
            // Place temporary obstruction
            aoc::matset(&mut matrix, loc, '#');
            let result = guard_walk_loop(guard_pos.into(), &matrix, '#');
            if result.1 == true { possible_obsts += 1 }
            // Reset to previous value
            aoc::matset(&mut matrix, loc, was);
        }

        println!("{}", possible_obsts);
    }
}
