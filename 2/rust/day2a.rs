use std::fs;

fn main() {
    let sample: &str = "7 6 4 2 1\n\
    1 2 7 8 9\n\
    9 7 6 2 1\n\
    1 3 2 4 5\n\
    8 6 4 4 1\n\
    1 3 6 7 9\n";

    let puzzle: String = fs::read_to_string("../input.txt").unwrap();

    let lines: Vec<&str> = puzzle.trim().split("\n").collect();
    let mut reports: Vec<Vec<i32>> = vec![];

    for l in lines {
        reports.push(l.split_whitespace().map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        );
    }

    let mut safe_reps: i32 = reports.len() as i32;

    for rep in reports {
        let mut init_pattern: &str = "";
        for n in 0..rep.len() {
            if n + 1 == rep.len() { break; }
            let a: i32 = rep[n];
            let b: i32 = rep[n + 1];

            if !(1..4).contains(&(a - b).abs()) { safe_reps -= 1; break; }

            let this_pattern = if a < b { "increasing" } else { "decreasing" };
            if n == 0 { init_pattern = &this_pattern; }
            if &init_pattern != &this_pattern { safe_reps -= 1; break; }
        }
    }

    println!("{}", safe_reps);
}
