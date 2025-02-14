use std::{env, fs, process};

#[path = "../../utils/aoc.rs"]
mod aoc;

#[derive(Debug)]
struct Disk {
    blocks: Vec<DiskBlock>
}

#[derive(Debug)]
struct DiskBlock {
    is_free: bool,
    id: u32,
    size: u32
}

impl Disk {
    fn empty() -> Disk {
        Disk{blocks: vec![]}
    }

    fn from_diskmap(diskmap: &str) -> Disk {
        let mut disk: Disk = Disk::empty();

        let mut mode: i32 = 1;
        let mut fblocks: u32 = 0;
        let mut ublocks: u32 = 0;

        for ch in diskmap.trim().chars() {
            let digit: u32 = ch.to_digit(10).unwrap();
            disk.blocks.push(DiskBlock{
                is_free: mode == -1,
                id: if mode == 1 { ublocks } else { fblocks },
                size: digit
            });

            if mode == 1 { ublocks += 1 } else { fblocks += 1 }
            mode *= -1;
        }

        disk
    }

    fn as_string(&self) -> String {
        let mut diskstr: String = "".into();
        for bl in self.blocks.iter() {
            let ch: String = if bl.is_free { ".".to_string() } else { bl.id.to_string() };
            for _ in 0..bl.size { diskstr.push_str(ch.as_str()) }
        }

        diskstr
    }

    fn free(&self) -> Vec<(usize, &DiskBlock)> {
        let mut fblocks: Vec<(usize, &DiskBlock)> = vec![];
        for (n, bl) in self.blocks.iter().enumerate() {
            if bl.is_free { fblocks.push((n, bl)) }
        }

        fblocks
    }

    fn used(&self) -> Vec<(usize, &DiskBlock)> {
        let mut fblocks: Vec<(usize, &DiskBlock)> = vec![];
        for (n, bl) in self.blocks.iter().enumerate() {
            if !bl.is_free { fblocks.push((n, bl)) }
        }

        fblocks
    }

    // i'm sure there's a better name for this but i'm not sure what i'd use...
    fn optimize_block(&mut self, ublock_index: usize, defrag: bool) -> Option<(usize, &DiskBlock)> {
        let mut used_block: &mut DiskBlock = &mut self.blocks[ublock_index];
        let mut fblocks = self.used();
        if fblocks[0].0 > ublock_index { return None }

        let mut new_block: DiskBlock = DiskBlock{is_free: false, id: used_block.id, size: 0};
        let mut new_block_index: usize = fblocks[0].0;
        self.blocks.insert(new_block_index, new_block);

        let mut nearest_free = &mut fblocks[0];

        while used_block.size > 0 {
            used_block.size -= 1;
            new_block.size += 1;
            nearest_free.1.size -= 1;
        }

        Some((new_block_index, &new_block))
    }
}

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
    let mut disk = Disk::from_diskmap(&puzzle_input);
    dbg!(disk.used());
    dbg!(disk.as_string());
}
