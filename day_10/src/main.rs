use std::fs::File;
use std::io::{prelude::*, BufReader};
use itertools::Itertools;
use num::pow;

fn parse_file(path: &str) -> Vec<u64> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let res: Vec<u64> = reader.lines()
        .map(|x|
            x.unwrap().parse::<u64>().unwrap())
        .collect();
    res
}

fn get_sorted_adapter_list(input: &mut Vec<u64>) {
    input.push(0);
    input.sort();
    input.push(input[input.len() - 1] + 3);
}

fn part_1() {
    let mut adapter_list = parse_file("/home/jeremy/github/aoc_2020/day_10/input/sample_2.txt");
    get_sorted_adapter_list(&mut adapter_list);

    let sub: Vec<u64> = adapter_list.windows(2)
        .map(|win|
            (win.get(1).unwrap() - win.get(0).unwrap()))
        .collect();

    let count_1 = sub.iter().filter(|&x| *x == 1).count();
    let count_2 = sub.iter().filter(|&x| *x == 2).count();
    let count_3 = sub.iter().filter(|&x| *x == 3).count();

    println!("{} * {} = {}", count_1, count_3, count_1 * count_3);
}

fn check_list(input: &Vec<u64>, win_size: usize) -> bool {
    let count = input.windows(win_size)
        .map(|win|
            (*win.get(win_size - 1).unwrap() - *win.get(0)
                .unwrap()))
        .filter(|x|
            *x > 3)
        .count();

    if count >= 1 {
        false
    } else {
        true
    }
}

fn main() {
    //part_1();
    let mut adapter_list = parse_file("/home/jeremy/github/aoc_2020/day_10/input/input_1.txt");


    adapter_list.push(0);
    adapter_list.sort();
    let max = adapter_list[adapter_list.len() - 1] + 3;
    adapter_list.push(max);
    adapter_list.sort();

    let mut difference = Vec::new();
    for idx in 0..(adapter_list.len() - 1) {
        let upper = adapter_list[idx + 1] - adapter_list[idx];
        println!("{}", upper);
        difference.push(upper);
    }

    let mut one_count = 0;

    let mut two = 0;
    let mut three = 0;
    let mut four = 0;

    for idx in 0..difference.len() {
        if difference[idx] == 1 {
            one_count += 1;
        } else {
            println!("One Count: {}", one_count);
            if one_count == 4 {
                four += 1;
            } else if one_count == 3 {
                three += 1
            } else if one_count == 2 {
                two += 1;
            }
            one_count = 0;
        }
    }
    let mut res = 1;
    res *= pow(2u64, two);
    res *= pow(4u64, three);
    res *= pow(7u64, four);

    println!("result: {}", res)
}

