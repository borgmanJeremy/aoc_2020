use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
    heading: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    magnitude: i32,
}

fn parse_file(path: &str) -> Vec<Instruction> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut direction_list: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let num = &text[1..text.len()].parse::<i32>().unwrap();
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


fn move_boat(current_pos: &Coordinate, inst: Instruction) -> Coordinate {
    match inst.dir {
        Direction::North => { Coordinate { x: current_pos.x, y: current_pos.y + inst.magnitude, heading: current_pos.heading } }
        Direction::East => { Coordinate { x: current_pos.x + inst.magnitude, y: current_pos.y, heading: current_pos.heading } }
        Direction::South => { Coordinate { x: current_pos.x, y: current_pos.y - inst.magnitude, heading: current_pos.heading } }
        Direction::West => { Coordinate { x: current_pos.x - inst.magnitude, y: current_pos.y, heading: current_pos.heading } }
        _ => { panic!("Command not found") }
    }
}

fn rotate_boat(current_pos: &Coordinate, inst: Instruction) -> Coordinate {
    let current_rot: i32 =
        match current_pos.heading {
            Direction::North => { 0 }
            Direction::East => { 90 }
            Direction::South => { 180 }
            Direction::West => { 270 }
            _ => { panic!("Invalid Heading") }
        };

    let rot = match inst.dir {
        Direction::Left => { -inst.magnitude }
        Direction::Right => { inst.magnitude }
        _ => { panic!("Invalid Rotation") }
    };

    let mut new_rot = (current_rot + rot) % 360;

    loop {
        if new_rot > 360 {
            new_rot -= 360;
        } else { break; }
    }
    loop {
        if new_rot < 0 {
            new_rot += 360;
        } else { break; }
    }


    let new_heading = match new_rot {
        0 => { Direction::North }
        90 => { Direction::East }
        180 => { Direction::South }
        270 => { Direction::West }
        _ => { panic!("Invalid new rotation") }
    };

    Coordinate { x: current_pos.x, y: current_pos.y, heading: new_heading }
}


fn process_instruction(current_pos: &Coordinate, inst: Instruction) -> Coordinate {
    match inst.dir {
        Direction::North => { move_boat(&current_pos, inst) }
        Direction::East => { move_boat(&current_pos, inst) }
        Direction::South => { move_boat(&current_pos, inst) }
        Direction::West => { move_boat(&current_pos, inst) }
        Direction::Forward => { move_boat(&current_pos, Instruction { magnitude: inst.magnitude, dir: current_pos.heading }) }
        Direction::Left => { rotate_boat(&current_pos, inst) }
        Direction::Right => { rotate_boat(&current_pos, inst) }
    }
}

fn main() {
    let inst_list = parse_file("/home/jeremy/github/aoc_2020/day_12/input/input_1.txt");
    let mut new_pos = Coordinate { x: 0, y: 0, heading: Direction::East };

    for inst in inst_list {
        new_pos = process_instruction(&new_pos.clone(), inst);
        println!("New Pos: {:?}", new_pos);
    }

    println!("Final Pos: {:?}", new_pos);
    println!("Distance: {:?}", new_pos.x.abs() + new_pos.y.abs());
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_rotate_boat() {
        assert_eq!(
            rotate_boat(
                &Coordinate {
                    x: 0,
                    y: 0,
                    heading: Direction::North,
                },
                Instruction {
                    dir: Direction::Right,
                    magnitude: 90,
                }),
            Coordinate {
                x: 0,
                y: 0,
                heading: Direction::East,
            });


        assert_eq!(
            rotate_boat(
                &Coordinate {
                    x: 0,
                    y: 0,
                    heading: Direction::North,
                },
                Instruction {
                    dir: Direction::Left,
                    magnitude: 90,
                }),
            Coordinate {
                x: 0,
                y: 0,
                heading: Direction::West,
            });

        assert_eq!(
            rotate_boat(
                &Coordinate {
                    x: 0,
                    y: 0,
                    heading: Direction::West,
                },
                Instruction {
                    dir: Direction::Right,
                    magnitude: 90,
                }),
            Coordinate {
                x: 0,
                y: 0,
                heading: Direction::North,
            });


    }
}


