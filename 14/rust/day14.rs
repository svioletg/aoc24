use std::{collections::HashMap, env, fs, process};

use aoc::MxPoint;

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("14/sample.txt").unwrap();
    #[allow(unused_variables)]
    let sample_rm: (isize, isize) = (11, 7);
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("14/input.txt").unwrap();
    #[allow(unused_variables)]
    let real_rm: (isize, isize) = (101, 103);

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day 14, Part {}", puzzle_part);

    solve_puzzle((real_in, real_rm), puzzle_part);
}

struct Robot {
    pos: MxPoint,
    vel: MxPoint
}

struct RobotRoom {
    robots: Vec<Robot>,
    w: isize,
    h: isize
}

impl RobotRoom {
    fn advance_sec(&mut self) {
        for rob in self.robots.iter_mut() {
            let mut future: MxPoint = rob.pos + rob.vel;
            if future.0 < 0 { future.0 += self.w }
            if future.0 >= self.w { future.0 -= self.w }
            if future.1 < 0 { future.1 += self.h }
            if future.1 >= self.h { future.1 -= self.h }
            rob.pos = future;
        }
    }

    fn count_robots(&self) -> HashMap<MxPoint, usize> {
        let mut count: HashMap<MxPoint, usize> = HashMap::new();
        for rob in self.robots.iter() {
            count.insert(rob.pos, count.get(&rob.pos).unwrap_or(&0) + 1);
        }

        count
    }

    fn safety_factor(&self) -> u32 {
        let topleft     = (0..self.w / 2,            0..self.h / 2);
        let topright    = ((self.w / 2) + 1..self.w, 0..self.h / 2);
        let bottomleft  = (0..self.w / 2,            self.h / 2 + 1..self.h);
        let bottomright = (self.w / 2 + 1..self.w,   self.h / 2 + 1..self.h);
        let mut quadcounts: [u32; 4] = [0; 4];

        for (idx, quadrange) in [topleft, topright, bottomleft, bottomright].iter().enumerate() {
            for rob in self.robots.iter() {
                if quadrange.0.contains(&rob.pos.0) && quadrange.1.contains(&rob.pos.1) {
                    quadcounts[idx] += 1;
                }
            }
        }

        quadcounts.iter().product()
    }

    fn from(robotstr: &str, w: isize, h: isize) -> RobotRoom {
        let mut robots: Vec<Robot> = vec![];
        for line in robotstr.split('\n') {
            if line.trim().is_empty() { continue }
            let parsed: Vec<Vec<isize>> = line.split_whitespace().map(|i| {
                i.split('=')
                .collect::<Vec<&str>>()
                [1].split(',')
                .filter_map(|i| i.parse::<isize>().ok())
                .collect::<Vec<isize>>()
            }).collect();
            robots.push(Robot{pos: MxPoint(parsed[0][0], parsed[0][1]), vel: MxPoint(parsed[1][0], parsed[1][1])});
        }

        RobotRoom{robots, w, h}
    }
}

fn solve_puzzle(puzzle_input: (String, (isize, isize)), puzzle_part: i32) {
    let mut rm = RobotRoom::from(&puzzle_input.0, puzzle_input.1.0, puzzle_input.1.1);
    for _ in 0..100 { rm.advance_sec() }
    println!("{}", rm.safety_factor());
}
