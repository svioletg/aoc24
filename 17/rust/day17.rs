use std::{env, fs, process};

fn main() {
    #[allow(unused_variables)]
    let sample_in: String = fs::read_to_string("17/sample.txt").unwrap();
    #[allow(unused_variables)]
    let real_in: String = fs::read_to_string("17/input.txt").unwrap();

    let args: Vec<String> = env::args().collect();
    let puzzle_part: i32 = match args[1].as_str() {
        "a" => 1,
        "b" => 2,
        _ => {
            println!("Argument for multi-part day solution must either \"a\" for part one, or \"b\" for part two.");
            process::exit(1);
        }
    };

    println!("Day 17, Part {}", puzzle_part);

    solve_puzzle(real_in, puzzle_part);
}

#[derive(Debug)]
struct ElfComputer {
    r_a: u32,
    r_b: u32,
    r_c: u32,
    program: Vec<u8>,
    stdout: Vec<u32>,
    inst_ptr: u32
}

impl ElfComputer {
    fn from_str(init_str: &str) -> ElfComputer {
        let mut r_a: u32 = 0;
        let mut r_b: u32 = 0;
        let mut r_c: u32 = 0;
        let mut program: Vec<u8> = vec![];
        for line in init_str.split('\n') {
            if line.contains("Register A:") {
                r_a = line.split("Register A:").find(|i| !i.is_empty()).unwrap().trim().parse::<u32>().unwrap();
            } else if line.contains("Register B:") {
                r_b = line.split("Register B:").find(|i| !i.is_empty()).unwrap().trim().parse::<u32>().unwrap();
            } else if line.contains("Register C:") {
                r_c = line.split("Register C:").find(|i| !i.is_empty()).unwrap().trim().parse::<u32>().unwrap();
            } else if line.contains("Program:") {
                program = line.split("Program:").find(|i| !i.is_empty()).unwrap()
                    .split(',')
                    .filter_map(|i| i.trim().parse::<u8>().ok())
                    .collect();
            }
        }

        ElfComputer {r_a, r_b, r_c, stdout: vec![], program, inst_ptr: 0}
    }

    fn print_stdout(&self) {
        println!("{}", self.stdout.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","));
    }

    fn run_program(&mut self) -> &Vec<u32> {
        while self.inst_ptr < self.program.len() as u32 {
            self.instruction(self.program[self.inst_ptr as usize], self.program[self.inst_ptr as usize + 1] as u32);
        }

        &self.stdout
    }

    fn combo_op(&self, operand: u32) -> u32 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.r_a,
            5 => self.r_b,
            6 => self.r_c,
            7 => panic!("Reserved operand used"),
            _ => panic!("Invalid value for combo operand")
        }
    }

    fn instruction(&mut self, opcode: u8, operand: u32) {
        match opcode {
            0 => {
                // adv
                let operand = self.combo_op(operand);
                self.r_a = self.r_a / (2_u32.pow(operand));
                self.inst_ptr += 2;
            },
            1 => {
                // bxl
                self.r_b = self.r_b ^ operand;
                self.inst_ptr += 2;
            },
            2 => {
                // bst
                let operand = self.combo_op(operand);
                self.r_b = operand % 8;
                self.inst_ptr += 2;
            },
            3 => {
                // jnz
                if self.r_a == 0 { self.inst_ptr += 2; }
                else { self.inst_ptr = operand; }
            },
            4 => {
                // bxc
                self.r_b = self.r_b ^ self.r_c;
                self.inst_ptr += 2;
            }
            5 => {
                // out
                let operand = self.combo_op(operand);
                self.stdout.push(operand % 8);
                self.inst_ptr += 2;
            },
            6 => {
                // bdv
                let operand = self.combo_op(operand);
                self.r_b = self.r_a / (2_u32.pow(operand));
                self.inst_ptr += 2;
            },
            7 => {
                // cdv
                let operand = self.combo_op(operand);
                self.r_c = self.r_a / (2_u32.pow(operand));
                self.inst_ptr += 2;
            },
            _ => panic!("Invalid opcode")
        }
    }
}

fn solve_puzzle(puzzle_input: String, puzzle_part: i32) {
    let mut cpu = ElfComputer::from_str(&puzzle_input);
    cpu.run_program();
    cpu.print_stdout();
}
