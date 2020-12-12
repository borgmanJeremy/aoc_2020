use std::fs::File;
use std::io::{prelude::*, BufReader};



#[derive(Debug)]
struct Coordinate{
    x: u32,
    y: u32
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}


#[derive(Debug)]
struct Instruction {
    dir: Direction,
    magnitude: u32,
}

fn parse_file(path: &str) -> Vec<Instruction> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut direction_list: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let num = &text[1..text.len()].parse::<u32>().unwrap();
        let direction = match text.chars().nth(0).unwrap() {
            'N' => Direction::North,
            'E' => Direction::East,
            'S' => Direction::South,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => {
                panic!("Char not found")
            }
        };
        direction_list.push(Instruction {
            dir: direction,
            magnitude: *num,
        });
    }
    direction_list
}

fn main() {
    let inst_list = parse_file("/home/jeremy/github/aoc_2020/day_12/input/sample.txt");
    let mut pos = Coordinate{x:0, y:0};

    for inst in inst_list{

    }
}
