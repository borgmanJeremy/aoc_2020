use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct BagWithCount {
    parent: String,
    child: HashMap<String, i32>,
}

#[derive(Debug)]
struct Bag {
    parent: String,
    child: Vec<String>,
}

fn find_parents(bag_list: &Vec<Bag>, input: &str) -> Vec<String> {
    let mut parent_list = Vec::new();
    for bag in bag_list {
        for child in &bag.child {
            if child == input {
                parent_list.push(bag.parent.clone());
            }
        }
    }

    parent_list
}

fn parse_bag_list(path: &str) -> Vec<Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<String>> = reader.lines()
        .map(|line| {
            line.unwrap()
                .replace("bags", "")
                .replace("bag", "")
                .replace(".", "")
        })
        .map(|token| {
            token
                .split("contain")
                .map(|x| { String::from(x.trim()) })
                .collect()
        })
        .collect();
    return lines;
}

fn part_1(path: &str) {
    let lines = parse_bag_list(path);
    let mut bag_list = Vec::new();
    for tokens in &lines {
        let mut bag = Bag {
            parent: String::from(&tokens[0]),
            child: Vec::new(),
        };

        let child_list: Vec<&str> = tokens[1].split(",")
            .map(|x| { x.trim() })
            .collect();
        for child in child_list {
            bag.child.push(String::from(&child[2..child.len()]));
        }
        bag_list.push(bag);
    }


    let mut parent_list = HashSet::new();
    parent_list.insert(String::from("shiny gold"));

    let mut found_new = true;
    while found_new {
        let mut new_parent: Vec::<String> = Vec::new();
        found_new = false;
        for parent_bag in &parent_list {
            let new_bag = find_parents(&bag_list, &parent_bag);

            for child_bag in new_bag {
                new_parent.push(child_bag);
            }
        }

        for bag in new_parent {
            match parent_list.insert(bag) {
                true => { found_new = true; }
                false => {}
            }
        }
    }
    println!("{:?}", parent_list.len() - 1);
}

fn get_children(parent: String, list: &Vec<BagWithCount>) -> Vec<HashMap<String, i32>> {
    let mut child_list = Vec::new();
    let mut found = false;

    for bag in list
    {
        if bag.parent == parent {
            child_list.push(bag.child.clone());
            found = true;
        }
    }

    child_list
}

fn walk_values(initial_value: String, count: i32,  sum: &mut i32, list: &Vec<BagWithCount>){
    let value = get_children(initial_value, &list);
    let mut total = 0;
    for map in value {
        if !map.is_empty() {
            for (key, value) in map.into_iter() {
                total += value*count;
                *sum =(value*count + *sum);
                walk_values(key.clone(), count * value, sum, &list);
                //println!("({}, {}), {}", key, value,count*value );
            }
        }
        else {
        }

    }

}

fn part_2(path: &str) {
    let lines = parse_bag_list(path);
    let mut bag_list = Vec::new();
    for tokens in &lines {
        let mut bag = BagWithCount {
            parent: String::from(&tokens[0]),
            child: HashMap::new(),
        };

        let child_list: Vec<&str> = tokens[1].split(",")
            .map(|x| { x.trim() })
            .collect();
        for child in child_list {
            if child != "no other" {
                bag.child.insert(String::from(&child[2..child.len()]), (child.as_bytes()[0] - 0x30) as i32);
            }
        }
        bag_list.push(bag);
    }

    let mut total = 0;
     walk_values(String::from("shiny gold"), 1,&mut total, &bag_list);
    println!("{:?}", total);
}

fn main() {
    part_1("/home/jeremy/github/aoc_2020/day_7/input/input_1.txt");
    part_1("/home/jeremy/github/nic_aoc/data/raw/day7_input.txt");
    part_2("/home/jeremy/github/aoc_2020/day_7/input/input_1.txt");
}
