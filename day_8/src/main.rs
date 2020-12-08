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


fn main() {
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