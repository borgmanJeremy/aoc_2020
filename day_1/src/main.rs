use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_from_file(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => {
                data.push(text.parse::<i32>().unwrap());
            }
            Err(err) => { println!("Error reading line:{} ", err); }
        }
    }
    return data;
}

fn main() {
    println!("Hello, world!");
    let mut input_data = read_from_file("/home/jeremy/github/aoc_2020/day_1/input/input_1.txt");
    input_data.sort();

    for num in &input_data {
        let elem = 2020 - num;
        match input_data.binary_search(&elem) {
            Ok(idx) => {
                println!("{}*{}={}", num, input_data[idx], num * input_data[idx])
            }
            Err(_err) => {}
        }
    }
}