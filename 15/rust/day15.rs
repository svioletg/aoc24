use std::{env, fs, process};

use aoc::{Matrix, MxPoint, MxPointDirections};

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("15/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("15/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day 15, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

fn parse_input(puzzle_in: &str) -> (Matrix<char>, Vec<MxPoint>, MxPoint) {
    let mut warehouse_map: Matrix<char> = Matrix::new();
    let mut robot_moves: Vec<MxPoint> = vec![];
    let mut robot_pos: Option<MxPoint> = None;

    let mut parse_part: u32 = 1;
    for line in puzzle_in.split('\n') {
        if line.trim().is_empty() { parse_part += 1; continue }
        if parse_part == 1 {
            warehouse_map.rows.push(line.chars().collect());
            if line.contains('@') {
                robot_pos = Some(MxPoint(
                    warehouse_map.height() as isize - 1,
                    line.chars().position(|i| i == '@').unwrap() as isize
                ));
            }
        } else if parse_part == 2 {
            robot_moves.extend(line.chars().map(|i| MxPoint::char_cardinal(&i).unwrap()));
        }
    }

    (warehouse_map, robot_moves, robot_pos.expect("Can\'t find robot in input!"))
}

fn predict_robot(src_mat: &Matrix<char>, instructions: &Vec<MxPoint>, robot_start: MxPoint) -> Matrix<char> {
    let mut mat: Matrix<char> = src_mat.clone();

    fn check_boxes(mat: &mut Matrix<char>, box_pos: MxPoint, dirpt: MxPoint) -> Option<MxPoint> {
        let box_future: MxPoint = box_pos + dirpt;
        let mut ret: Option<MxPoint> = None;
        match mat.get(box_future) {
            Some('#') => ret = None,
            Some('.') => ret = Some(box_future),
            Some('O') => ret = check_boxes(mat, box_future, dirpt),
            Some(other) => panic!("Unexpected character at {}: {}", box_future, other),
            None => panic!("Unexpectedly out of bounds at {}", box_future)
        }

        ret
    }

    let mut robot_pos = robot_start;
    for dirpt in instructions {
        let robot_future = robot_pos + *dirpt;
        match mat.get(robot_future) {
            Some('#') => { continue},
            Some('.') => {
                mat.set(robot_pos, '.');
                robot_pos = robot_future;
                mat.set(robot_pos, '@');
            },
            Some('O') => {
                let result = check_boxes(&mut mat, robot_future, *dirpt);
                if result.is_some() {
                    mat.set(robot_pos, '.');
                    robot_pos = robot_future;
                    mat.set(robot_pos, '@');
                    mat.set(result.unwrap(), 'O');
                }
            },
            Some(other) => panic!("Unexpected character at {}: {}", robot_future, other),
            None => panic!("Unexpectedly out of bounds at {}", robot_future)
        }
    }

    mat
}

fn box_gps_coords(warehouse: &Matrix<char>) -> Vec<u32> {
    let mut coords: Vec<u32> = vec![];
    for (ridx, row) in warehouse.rows.iter().enumerate() {
        for (cidx, column) in row.iter().enumerate() {
            if *column == 'O' { coords.push((100 * ridx as u32) + cidx as u32); }
        }
    }

    coords
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let (warehouse, robot_moves, robot) = parse_input(&puzzle_input);
    let predicted = predict_robot(&warehouse, &robot_moves, robot);
    println!("{}", box_gps_coords(&predicted).iter().sum::<u32>());
}
