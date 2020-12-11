use std::fs::File;
use std::io::{prelude::*, BufReader};

enum SeatType {
    FLOOR,
    EMPTY,
    FULL,
}

fn parse_file(path: &str) -> Vec<Vec<SeatType>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut seat_map: Vec<Vec<SeatType>> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let mut row: Vec<SeatType> = Vec::new();

        for char in text.chars() {
            match char {
                '.' => row.push(SeatType::FLOOR),
                'L' => row.push(SeatType::EMPTY),
                '#' => row.push(SeatType::FULL),
                _ => {
                    panic!("Char not found")
                }
            }
        }
        seat_map.push(row);
    }
    seat_map
}

fn print_map(seat_map: &Vec<Vec<SeatType>>) {
    for row in seat_map {
        for seat in row {
            match seat {
                SeatType::FLOOR => {
                    print!(".")
                }
                SeatType::EMPTY => {
                    print!("L")
                }
                SeatType::FULL => {
                    print!("#")
                }
            }
        }
        println!("");
    }
}

fn main() {
    let seat_map = parse_file("/home/jeremy/github/aoc_2020/day_11/input/sample.txt");

    //print_map(&seat_map);
}

/*

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.
 */
