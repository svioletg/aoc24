use std::{env, fs, process};

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("<N>/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("<N>/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day <N>, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {

}
