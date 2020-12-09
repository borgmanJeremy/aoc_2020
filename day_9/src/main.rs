use std::fs::File;
use std::io::{prelude::*, BufReader};
use itertools::Itertools;

fn parse_file(path: &str) -> Vec<u64> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let res: Vec<u64> = reader.lines()
        .map(|x|
            x.unwrap().parse::<u64>().unwrap())
        .collect();
    res
}

fn find_pair(search_range: &Vec<u64>, target: u64) -> Option<(u64, u64)> {
    let combo = search_range.into_iter().combinations(2).collect_vec();

    let filtered: Vec<Vec<&u64>> = combo.into_iter().filter(
        |x|
            *x[0] + *x[1] == target)
        .collect();

    if filtered.is_empty() {
        return None;
    }

    Some((*filtered[0][0], *filtered[0][1]))
}

fn main() {
    let num_list = parse_file("/home/jeremy/github/aoc_2020/day_9/input/input_1.txt");

    let win_size = 25;


    for idx in 0..num_list.len() - win_size {
        let search_range = num_list[idx..idx + win_size].to_vec();
        let res = find_pair(&search_range, num_list[idx + win_size]);
        match res {
            None => { println!("target that failed: {}", num_list[idx + win_size]) }
            Some(_) => {}
        }
    }
}
