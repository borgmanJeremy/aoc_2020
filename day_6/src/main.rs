use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
//use boardingpass::*;


fn part_1(path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut declaration = HashSet::new();
    for line in reader.lines() {
        let text = line.unwrap();
        if text.is_empty() {
            total += declaration.len();
            //println!("yes answers: {}", declaration.len());
            declaration.clear();
        } else {
            for char in text.chars() {
                declaration.insert(char);
            }
        }
    }

    println!("total is: {}", total);
}


fn main() {
    part_1("/home/jeremy/github/aoc_2020/day_6/input/input_1.txt");
}
