use std::fs::File;
use std::io::{prelude::*, BufReader};
use boardingpass::*;


fn read_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut boardingpass_list = Vec::new();
    for line in reader.lines() {
        boardingpass_list.push(line.unwrap());
    }

    return boardingpass_list;
}

fn find_missing_num(input: &Vec<i32>) -> Option<i32> {
    let mut orig_elem = input[0];
    let mut new_elem = input[0];

    for idx in 1..(input.len() - 1) {
        new_elem = input[idx];
        if new_elem - orig_elem != 1
        {
            return Some(new_elem-1);
        }
        orig_elem = new_elem;
    }
    None
}

fn main() {
    let boardingpass_list = read_from_file("/home/jeremy/github/aoc_2020/day_5/input/input_1.txt");

    let mut max = 0;
    let mut seat_id_list = Vec::new();

    for pass in boardingpass_list {
        let seat_id = seat_id(&pass);
        if seat_id > max {
            max = seat_id;
        }
        seat_id_list.push(seat_id);
    }
    seat_id_list.sort();

    println!("The maximum seat id is: {} ", max);

    let missing_seat = find_missing_num(&seat_id_list);

    match missing_seat {
        Some(seat) => { println!("Missing seat is: {}", seat) }
        None => println!("Seat Not Found")
    }
}
