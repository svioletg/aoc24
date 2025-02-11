use std::{env, fs, process};

fn main() {
    let sample_in: String = fs::read_to_string("../sample.txt").unwrap();
    let real_in: String = fs::read_to_string("../input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let mut puzzle_part: i32 = 0;

    if args[1] == "a" {
        puzzle_part = 1;
    } else if args[1] == "b" {
        puzzle_part = 2;
    } else {
        println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
        process::exit(1);
    }

    solve_puzzle(real_in, puzzle_part);
}

#[derive(Debug)]
struct ManualUpdate {
    content: Vec<i32>,
    needed_correction: bool,
}

#[derive(Debug)]
struct UpdateValidationResult {
    valid: i32,
    invalid: i32
}

/// Checks each of `updates` to see if their page orders are valid according to `ordering_rules`.
/// If `fix` is `true`, the updates will be corrected in-place.
///
/// Returns an `UpdateValidationResult`, where `valid` is the number of updates that were already in the correct order,
/// and `invalid` is the number of updates that were not. `fix` does not alter these values; if the updates were corrected
/// by this function, `invalid` represents the number of updates that *were* ordered incorrectly before being rearranged.
fn validate_update(ordering_rules: &Vec<(i32, i32)>, update_group: &mut Vec<ManualUpdate>, fix: bool) -> UpdateValidationResult {
    let mut items_valid: i32 = 0;
    let mut items_invalid: i32 = 0;

    for u in update_group.iter_mut() {
        let mut was_invalid: bool = false;
        let mut order_is_correct: bool = false;
        while !order_is_correct {
            let mut moves: i32 = 0;
            for (r_a, r_b) in ordering_rules.clone() {
                let a_pos: usize = match u.content.iter().position(|&i| i == r_a) {
                    Some(val) => val,
                    None => continue
                };
                let b_pos: usize = match u.content.iter().position(|&i| i == r_b) {
                    Some(val) => val,
                    None => continue
                };

                if a_pos > b_pos {
                    was_invalid = true;
                    if !fix { break; }
                    let popped: i32 = u.content.remove(b_pos);
                    u.content.push(popped);
                    moves += 1;
                }
            }
            order_is_correct = moves == 0;
        }
        u.needed_correction = was_invalid;
        if was_invalid { items_invalid += 1; }
        else { items_valid += 1; }
    }

    UpdateValidationResult{valid: items_valid, invalid: items_invalid}
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let mut rules: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<ManualUpdate> = vec![];

    for line in puzzle_input.split("\n") {
        if line.contains("|") {
            let rset: Vec<i32> = line.split("|").map(|i| i.parse::<i32>().unwrap()).collect();
            rules.push((rset[0], rset[1]));
        } else if line.contains(",") {
            updates.push(ManualUpdate{
                content: line.split(",").map(|i| i.parse::<i32>().unwrap()).collect(),
                needed_correction: false
            });
        }
    }
    let validation_result: UpdateValidationResult = validate_update(&rules, &mut updates, puzzle_part == 2);
    println!("Valid: {}, Invalid: {}", validation_result.valid, validation_result.invalid);

    let mut medians: Vec<i32> = vec![];
    for u in updates.iter().filter(|&i| if puzzle_part == 1 { !i.needed_correction } else { i.needed_correction }) {
        medians.push(u.content[u.content.len() / 2]);
    }

    let medsum: i32 = medians.iter().sum();
    println!("{medsum}");
}
