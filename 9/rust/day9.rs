use std::{collections::HashMap, env, fs, process, time::{Duration, Instant}};

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

    solve_puzzle(sample_in, puzzle_part);
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let disk: Vec<i32> = parse_diskmap(&puzzle_input);
    let optimized: Vec<i32> = optimize_disk(&disk, if puzzle_part == 1 { false } else { true} );
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

#[derive(Debug, Clone)]
struct DiskChunk {
    start: usize, // Where this chunk begins in the original disk vector
    file_id: i32, // -1 if free space
    size: usize
}

fn disk_as_chunks(disk: &Vec<i32>) -> Vec<DiskChunk> {
    let mut disk_chunks: Vec<DiskChunk> = vec![];
    let mut last_block: Option<i32> = None;
    for (n, block) in disk.iter().enumerate() {
        if last_block.is_some_and(|some| some == *block) {
            let mut last_chunk: DiskChunk = disk_chunks.pop().expect("Chunks vector was empty!");
            last_chunk.size += 1;
            disk_chunks.push(last_chunk);
        } else {
            disk_chunks.push(DiskChunk{start: n, file_id: *block, size: 1});
        }
        last_block = Some(*block);
    }

    disk_chunks
}

fn chunks_as_disk(disk_chunks: &Vec<DiskChunk>) -> Vec<i32> {
    let mut disk: Vec<i32> = vec![];
    let mut sorted_chunks = disk_chunks.clone();
    sorted_chunks.sort_by_key(|i| i.size);
    for chunk in sorted_chunks.iter() {
        disk.extend(vec![chunk.file_id; chunk.size]);
    }

    disk
}

fn optimize_disk(disk: &Vec<i32>, defrag: bool) -> Vec<i32> {
    let mut optimized_disk: Vec<i32> = vec![];
    let loop_timer = Instant::now();
    if defrag {
        let mut disk_chunks: Vec<DiskChunk> = disk_as_chunks(&disk);
        for n in (0..disk_chunks.len()).rev() {
            if disk_chunks[n].file_id == -1 { continue }

            let (left, right) = disk_chunks.split_at_mut(n);
            let this_chunk: &mut DiskChunk = &mut right[0];

            let first_free_index: Option<usize> = left.iter().position(|i| i.file_id == -1 && i.size >= this_chunk.size );
            if first_free_index.is_none() { continue }
            let first_free: &mut DiskChunk = &mut left[first_free_index.unwrap()];
            if first_free.start > this_chunk.start { continue }

            let displaced: DiskChunk = DiskChunk{start: this_chunk.start, file_id: -1, size: this_chunk.size};
            this_chunk.start = first_free.start;
            first_free.size -= this_chunk.size;
            if first_free.size > 0 { first_free.start += this_chunk.size }
            else {
                disk_chunks.push(displaced);
            }
        }

        optimized_disk = chunks_as_disk(&disk_chunks);
    } else {
        optimized_disk = disk.clone();
        let mut free_blocks: Vec<usize> = optimized_disk.iter().enumerate().filter_map(|(n, i)| (*i == -1).then(|| n)).rev().collect();
        for n in (0..disk.len()).rev() {
            if disk[n] == -1 { continue }
            let first_free: usize = free_blocks.pop().expect("No free block indexes left!");

            if first_free > n { break }
            optimized_disk.swap(n, first_free);
        }
    }
    println!("Optimization completed in {:.5?}s", loop_timer.elapsed().as_secs_f32());

    dbg!(disk_as_string(&optimized_disk));

    optimized_disk
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
