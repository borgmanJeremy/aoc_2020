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


fn main() {
    let boardingpass_list = read_from_file("/home/jeremy/github/aoc_2020/day_5/input/input_1.txt");

    let mut max =0;
    for pass in boardingpass_list {
        let seat_id = seat_id(&pass);
        if seat_id > max{
            max = seat_id;
        }
    }

    println!("The maximum seat id is: {} ",max);
}
