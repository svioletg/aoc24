use std::fs;

fn main() {
    let sample_in: String = fs::read_to_string("../sample.txt").unwrap();
    let real_in: String = fs::read_to_string("../input.txt").unwrap();

    solve_puzzle(sample_in);
}

fn solve_puzzle(puzzle_input: String) {
    let mut rules: Vec<Vec<i32>> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

    for line in puzzle_input.split("\n") {
        if line.contains("|") {
            rules.push(line.split("|").map(|i| i.parse::<i32>().unwrap()).collect());
        } else if line.contains(",") {
            updates.push(line.split(",").map(|i| i.parse::<i32>().unwrap()).collect());
        }
    }

    for u in updates {

    }
}
