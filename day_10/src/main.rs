use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_file(path: &str) -> Vec<u64> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let res: Vec<u64> = reader.lines()
        .map(|x|
            x.unwrap().parse::<u64>().unwrap())
        .collect();
    res
}

fn part_1(){
    let mut adapter_list = parse_file("/home/jeremy/github/aoc_2020/day_10/input/input_1.txt");
    adapter_list.push(0);
    adapter_list.sort();
    adapter_list.push(adapter_list[adapter_list.len() - 1] + 3);

    let sub: Vec<u64> = adapter_list.windows(2)
        .map(|win|
            (win.get(1).unwrap() - win.get(0).unwrap()))
        .collect();

    let count_1 = sub.iter().filter(|&x| *x == 1).count();
    let count_3 = sub.iter().filter(|&x| *x == 3).count();

    println!("{} * {} = {}", count_1, count_3, count_1 * count_3);
}

fn main() {
    part_1();
}

