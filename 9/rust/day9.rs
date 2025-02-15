use std::{env, fs, process, time::{Duration, Instant}};

#[path = "../../utils/aoc.rs"]
mod aoc;

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("9/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("9/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day 9, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let disk: Vec<i32> = parse_diskmap(&puzzle_input);
    let optimized: Vec<i32> = optimize_disk(&disk);
    println!("{}", disk_checksum(&optimized));
}

fn parse_diskmap(diskmap: &String) -> Vec<i32> {
    let mut disk: Vec<i32> = vec![];
    let mut mode: i32 = 1;
    let mut files: u32 = 0;
    for ch in diskmap.chars() {
        if !ch.is_digit(10) { continue }
        let digit: u32 = ch.to_digit(10).unwrap();
        if mode == 1 {
            disk.extend(vec![files as i32; digit as usize]);
            files += 1;
        } else {
            disk.extend(vec![-1; digit as usize]);
        }
        mode *= -1;
    }

    disk
}

fn optimize_disk(disk: &Vec<i32>) -> Vec<i32> {
    let mut diskcopy: Vec<i32> = disk.clone();

    let loop_timer = Instant::now();
    let mut free_blocks: Vec<usize> = diskcopy.iter().enumerate().filter_map(|(n, i)| (*i == -1).then(|| n)).rev().collect();
    for n in (0..disk.len()).rev() {
        if disk[n] == -1 { continue }
        let first_free: usize = free_blocks.pop().expect("No free block indexes left!");

        if first_free > n { break }
        diskcopy.swap(n, first_free);
    }
    println!("Loop done and took {:.5?}s", loop_timer.elapsed().as_secs_f32());

    diskcopy
}

fn disk_as_string(disk: &Vec<i32>) -> String {
    let mut diskstr: String = String::new();
    for block in disk.iter() {
        let blockstr: &str = &block.to_string();
        diskstr.push_str(if blockstr == "-1" { "." } else { blockstr });
    }

    diskstr
}

fn disk_checksum(disk: &Vec<i32>) -> usize {
    let mut checksum: usize = 0;
    for (n, block) in disk.iter().enumerate() {
        if *block == -1 { continue }
        checksum += n * (*block as usize);
    }

    checksum
}
