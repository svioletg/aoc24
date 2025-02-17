use std::{env, fs, process};

#[path = "../../utils/aoc.rs"]
mod aoc;

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("11/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("11/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day 11, Part {}", puzzle_part);

    solve_puzzle(sample_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let stones: Vec<usize> = puzzle_input.split_whitespace().filter_map(|i| i.parse::<usize>().ok()).collect();
    let mut stonecount: usize = 0;
    for st in stones {
        stonecount += count_blinks_for_stone(st, if puzzle_part == 1 { 25 } else { 75 });
    }
    println!("{}", stonecount);
}

fn digits(n: i32) -> usize {
    ((n as f32).log(10.0).floor() as usize) + 1
}

fn blink_once(value: usize) -> (usize, Option<usize>) {
    let text: String = value.to_string();
    if value == 0 {
        return (1, None);
    } else if text.len() % 2 == 0 {
        let middle: usize = text.len() / 2;
        let (left, right) = text.split_at(middle);
        return (left.parse::<usize>().unwrap(), Some(right.parse::<usize>().unwrap()));
    } else {
        return (value * 2024, None)
    }
}

fn count_blinks_for_stone(value: usize, remaining: usize) -> usize {
    let (left, right) = blink_once(value);

    if remaining == 1 { return if right.is_none() { 1 } else { 2 } }

    let mut count: usize = count_blinks_for_stone(left, remaining - 1);
    if right.is_some() {
        count += count_blinks_for_stone(right.unwrap(), remaining - 1);
    }

    count
}
