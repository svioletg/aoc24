use std::env;
use std::fs;

fn main() {
    let sample_in: String = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let real_in: String = fs::read_to_string("../input.txt").unwrap();

    solve_puzzle(real_in);
}

fn solve_puzzle(puzzle_input: String) {
    let valid_inst_chars: Vec<char> = vec![',', ')'];
    let inst_delim: char = ')';

    let mut inst_dos: Vec<usize> = vec![];
    let mut inst_donts: Vec<usize> = vec![];

    for m in puzzle_input.match_indices("do()")    { inst_dos.push(m.0); }
    for m in puzzle_input.match_indices("don't()") { inst_donts.push(m.0); }

    let mut usable_input: String = String::from("");
    let mut writing: bool = true;

    for (n, ch) in puzzle_input.chars().enumerate() {
        if inst_dos.contains(&n) { writing = true; }
        if inst_donts.contains(&n) { writing = false; }
        if writing { usable_input.push_str(&ch.to_string()); }
    }

    let mut inst_strings: Vec<String> = vec![];
    for m in usable_input.match_indices("mul(") {
        let mut inst: String = String::from("");
        for n in (m.0 + 4)..usable_input.len() {
            let ch: char = usable_input.chars().nth(n).unwrap();
            if ch == inst_delim { break; }

            if !ch.is_numeric() && !valid_inst_chars.contains(&ch) {
                inst = String::from("");
                break;
            }

            inst.push_str(&ch.to_string());
        }

        if inst.split(',').filter(|i| !i.is_empty()).collect::<Vec<&str>>().len() == 2 {
            inst_strings.push(inst);
        }
    }

    let mut total: i32 = 0;
    for s in inst_strings {
        let pair: Vec<i32> = s.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
        total += pair[0] * pair[1];
    }

    println!("{}", total);
}
