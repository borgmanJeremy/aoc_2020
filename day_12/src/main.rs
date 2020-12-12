use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
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
    command: Command,
    magnitude: i32,
}

#[derive(Debug)]
struct Waypoint {
    pos: Coordinate,
}

impl Waypoint {
    fn new() -> Self {
        Self {
            pos: Coordinate { x: 0, y: 0 },
        }
    }
}

#[derive(Debug)]
struct Boat {
    pos: Coordinate,
}

impl Boat {
    fn new() -> Self {
        Self {
            pos: Coordinate { x: 0, y: 0 }
        }
    }
}

#[derive(Debug)]
struct Navigator {
    waypoint: Waypoint,
    boat: Boat,
}

impl Navigator {
    fn new() -> Self {
        Self {
            waypoint: Waypoint::new(),
            boat: Boat::new(),
        }
    }

    fn move_waypoint(&mut self, inst: Instruction) {
        let new_pos = match inst.command {
            Command::North => { Coordinate { x: self.waypoint.pos.x, y: self.waypoint.pos.y + inst.magnitude } }
            Command::East => { Coordinate { x: self.waypoint.pos.x + inst.magnitude, y: self.waypoint.pos.y } }
            Command::South => { Coordinate { x: self.waypoint.pos.x, y: self.waypoint.pos.y - inst.magnitude } }
            Command::West => { Coordinate { x: self.waypoint.pos.x - inst.magnitude, y: self.waypoint.pos.y } }
            _ => { panic!("Command not found") }
        };
        self.waypoint.pos = new_pos;
    }

    fn rotate_waypoint(&mut self, inst: Instruction) {
        let rot_count = inst.magnitude / 90;

        let mut new_pos = Coordinate { x: self.waypoint.pos.x - self.boat.pos.x, y: self.waypoint.pos.y - self.boat.pos.y };

        match inst.command {
            Command::Right => {
                for _i in 0..rot_count {
                    let temp_pos = Coordinate { x: new_pos.y, y: -new_pos.x };
                    new_pos = temp_pos
                }
            }
            Command::Left => {
                for _i in 0..rot_count {
                    let temp_pos = Coordinate { x: -new_pos.y, y: new_pos.x };
                    new_pos = temp_pos
                }
            }
            _ => { panic!("not found") }
        }
        self.waypoint.pos.x = self.boat.pos.x + new_pos.x;
        self.waypoint.pos.y = self.boat.pos.y + new_pos.y;
    }

    fn move_boat(&mut self, inst: Instruction) {
        let vect = Coordinate { x: self.waypoint.pos.x - self.boat.pos.x, y: self.waypoint.pos.y - self.boat.pos.y };
        let delta = Coordinate { x: vect.x * inst.magnitude, y: vect.y * inst.magnitude };

        self.boat.pos.x = self.boat.pos.x + delta.x;
        self.boat.pos.y = self.boat.pos.y + delta.y;

        self.waypoint.pos.x = self.waypoint.pos.x + delta.x;
        self.waypoint.pos.y = self.waypoint.pos.y + delta.y;
    }

    fn process_instruction(&mut self, inst: Instruction) {
        match inst.command {
            Command::North => { self.move_waypoint(inst) }
            Command::East => { self.move_waypoint(inst) }
            Command::South => { self.move_waypoint(inst) }
            Command::West => { self.move_waypoint(inst) }
            Command::Forward => { self.move_boat(inst) }
            Command::Left => { self.rotate_waypoint(inst) }
            Command::Right => { self.rotate_waypoint(inst) }
        }
    }
}

fn parse_file(path: &str) -> Vec<Instruction> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut direction_list: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let num = &text[1..text.len()].parse::<i32>().unwrap();
        let command = match text.chars().nth(0).unwrap() {
            'N' => Command::North,
            'E' => Command::East,
            'S' => Command::South,
            'W' => Command::West,
            'L' => Command::Left,
            'R' => Command::Right,
            'F' => Command::Forward,
            _ => {
                panic!("Char not found")
            }
        };
        direction_list.push(Instruction {
            command: command,
            magnitude: *num,
        });
    }
    direction_list
}


fn main() {
    let inst_list = parse_file("/home/jeremy/github/aoc_2020/day_12/input/input_1.txt");

    let mut navigator = Navigator::new();
    navigator.waypoint.pos.x = 10;
    navigator.waypoint.pos.y = 1;

    for inst in inst_list {
        navigator.process_instruction(inst);
        println!("Waypoint: {:?}", navigator.waypoint.pos);
        println!("Boat Pos: {:?}", navigator.boat.pos);
        println!("");
    }

     println!("Distance: {:?}", navigator.boat.pos.x.abs() + navigator.boat.pos.y.abs());
}
