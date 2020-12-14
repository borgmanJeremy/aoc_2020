use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Debug, Clone)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
    floating_mask: u64,
}

#[derive(Debug)]
enum Command {
    Mask(Mask),
    Mem { addr: u64, val: u64 },
}

fn parse_file(path: &str) -> Vec<Command> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut command_list: Vec<Command> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let token_list: Vec<&str> = text.split('=').collect();
        if token_list[0].contains("mask") {
            let mut and_mask: u64 = 0xFFFFFFFFFFFFFFFF;
            let mut or_mask: u64 = 0;
            let mut floating_mask: u64 = 0;
            for (idx, char) in token_list[1].chars().enumerate() {
                match char {
                    'X' => { floating_mask |= (1 << (36 - idx as u64)); }
                    '1' => { or_mask |= (1 << (36 - idx as u64)); }
                    '0' => { and_mask ^= (1 << (36 - idx as u64)); }
                    _ => {}
                }
            }
            command_list.push(Command::Mask(Mask { and_mask, or_mask, floating_mask }));
        } else {
            let addr = (&token_list[0].trim()[4..token_list[0].len() - 2]).parse::<u64>().unwrap();
            let val = token_list[1].trim().parse::<u64>().unwrap();
            command_list.push(Command::Mem { addr, val });
        }
    }
    command_list
}

fn part_1() {
    let command_list = parse_file("/home/jeremy/github/aoc_2020/day_14/input/input_1.txt");

    let mut mem_map: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = Mask { and_mask: 0, or_mask: 0, floating_mask: 0 };

    for command in &command_list {
        match command {
            Command::Mask(mask) => {
                current_mask = mask.clone();
            }
            Command::Mem { val, addr } => {
                let mut temp_val: u64 = *val;
                temp_val |= current_mask.or_mask;
                temp_val &= current_mask.and_mask;
                mem_map.insert(*addr, temp_val);
            }
        }
    }

    let mut sum = 0;
    for (key, val) in mem_map {
        sum += val;
    }
    println!("sum: {}", sum);
}


fn main() {
    // part_1();


    let command_list = parse_file("/home/jeremy/github/aoc_2020/day_14/input/input_1.txt");

    let mut mem_map: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = Mask { and_mask: 0, or_mask: 0, floating_mask: 0 };


    let mut addr_list = HashSet::new();
    for command in &command_list {

        match command {
            Command::Mask(mask) => {
                current_mask = mask.clone();
            }
            Command::Mem { val, addr } => {

                addr_list.clear();
                addr_list.insert(*addr|current_mask.or_mask);
                for bit in 0..36 {
                    if ((1 << bit) & current_mask.floating_mask) != 0 {
                        let mut new_vals = HashSet::new();
                        for addr in &addr_list {
                            let mut temp_addr: u64 = addr | (1 << bit);
                            new_vals.insert(temp_addr);

                            temp_addr = *addr;
                            let temp_mask = 0xFFFFFFFFFFFFFFFF^(1 << (bit as u64));
                            temp_addr &= temp_mask;
                            new_vals.insert(temp_addr);

                        }
                        for val in new_vals{
                            addr_list.insert(val);
                        }
                    }
                }
                for addr in &addr_list{
                    mem_map.insert(*addr,*val);
                }

                //println!("{:?}", addr_list);
                //println!("{:?}", mem_map);

            }
        }
    }

    let mut sum = 0;
    for (key, val) in mem_map {
        sum += val;
    }
    println!("sum: {}", sum);
}