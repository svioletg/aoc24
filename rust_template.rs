use std::env;
use std::fs;

fn main() {
    let sample_in: String = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    // let real_in: String = fs::read_to_string("inputs/REPLACEME.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    if args[1] == "1" { part_one(sample_in); }
    else if args[1] == "2" { part_two(sample_in); }
    else { println!("Wrong argument, must be 1 or 2: {}", args[0]); }
}

fn part_one(puzzle_input: String) {
    println!("1 {}", puzzle_input);
}

fn part_two(puzzle_input: String) {
    println!("2 {}", puzzle_input);
}
