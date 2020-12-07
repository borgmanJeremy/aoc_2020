use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

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

fn part_1(path: &str) {
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

    println!("{:?}", parent_list.len()-1);
}

fn main() {
    part_1("/home/jeremy/github/aoc_2020/day_7/input/input_1.txt");
}
