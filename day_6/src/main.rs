use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;
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
            declaration.clear();
        } else {
            for char in text.chars() {
                declaration.insert(char);
            }
        }
    }

    println!("total is: {}", total);
}

fn part_2(path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut decl_list: Vec<HashSet<char>> = Vec::new();
    let mut intersection = HashSet::new();

    for line in reader.lines() {
        let text = line.unwrap();
        if text.is_empty() {
            intersection = decl_list.iter()
                .skip(1)
                .fold(decl_list[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                });
            total += intersection.len();
            println!("Set Length: {}", intersection.len());

            decl_list.clear();
        } else {
            let mut declaration = HashSet::new();
            for char in text.chars() {
                declaration.insert(char);
            }
            decl_list.push(declaration);
        }
    }

    println!("total: {}", total);
}


fn main() {
    part_2("/home/jeremy/github/aoc_2020/day_6/input/input_1.txt");
}
