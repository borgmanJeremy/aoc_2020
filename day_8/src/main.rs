use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Reg {
    acc: i32,
    pc: i32,
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: String,
    operand: i32,
}

#[derive(Debug)]
struct Cpu {
    registers: Reg,
    memory: Vec<Instruction>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Reg { acc: 0, pc: 0 },
            memory: Vec::new(),
        }
    }

    fn decode_opcode(&mut self, input: &Instruction) {
        match input.opcode.as_str() {
            "nop" => {
                self.registers.pc += 1;
            }
            "acc" => {
                self.registers.acc += input.operand;
                self.registers.pc += 1;
            }
            "jmp" => {
                self.registers.pc += input.operand;
            }
            _ => { panic!("Invalid Op Code") }
        }
    }

    pub fn step(&mut self) {
        let mem_address = self.registers.pc as usize;
        let instruction = self.memory[mem_address].clone();
        self.decode_opcode(&instruction);
    }
    pub fn load_program(&mut self, path: &str) {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let text = line.unwrap();
            let token: Vec<&str> = text.split(" ").collect();
            self.memory.push(
                Instruction {
                    opcode: String::from(token[0]),
                    operand: token[1].parse::<i32>().unwrap(),
                });
        }
    }
}

fn part_1() {
    let mut cpu = Cpu::new();
    cpu.load_program("/home/jeremy/github/aoc_2020/day_8/input/input_1.txt");

    let mut pc_history = Vec::new();
    pc_history.push(0);
    loop {
        let old_acc = cpu.registers.acc;
        cpu.step();
        if pc_history.contains(&cpu.registers.pc) {
            println!("Old Acc was {}", old_acc);
            break;
        }
        pc_history.push(cpu.registers.pc);
    }
}

fn detect_loop(cpu: &mut Cpu) -> bool {
    let mut pc_history = Vec::new();
    pc_history.push(0);

    let mut found_loop = false;

    loop {
        cpu.step();
        if pc_history.contains(&cpu.registers.pc) {
            found_loop = true;
            break;
        }
        if cpu.registers.pc == 626 {
            return false;
        }
        pc_history.push(cpu.registers.pc);
    }
    found_loop
}

fn part_2() {
    let mut cpu = Cpu::new();
    cpu.load_program("/home/jeremy/github/aoc_2020/day_8/input/input_1.txt");
    let orig_cpu = cpu.memory.clone();

    for idx in 0..cpu.memory.len() {
        match detect_loop(&mut cpu) {
            true => {
                cpu.memory = orig_cpu.clone();
                cpu.registers.acc = 0;
                cpu.registers.pc = 0;
                match cpu.memory[idx].opcode.as_str() {
                    "nop" => { cpu.memory[idx].opcode = String::from("jmp"); }
                    "jmp" => { cpu.memory[idx].opcode = String::from("nop"); }
                    _ => {}
                }
            }
            false => {
                println!("Acc Value is: {}",cpu.registers.acc);
                break;
            }
        }
    }
}

fn main() {
    part_1();
    part_2();
}