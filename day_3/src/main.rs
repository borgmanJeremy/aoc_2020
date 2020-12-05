use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


#[derive(Debug,PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug,PartialEq)]
pub struct Slope {
    pub run: i64,
    pub rise: i64,
}

pub fn parse_map_to_vector(path: &str) -> (Vec<Point>, usize, usize) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open  file: {}", why.to_string()),
    };
    let buff = BufReader::new(file);

    let mut line_count = 0;
    let mut point_vec = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in buff.lines() {
        let unwrap = line.unwrap();
        let char_array = unwrap.as_bytes();
        height += 1;
        width = char_array.len();
        for idx in 0..char_array.len() {
            if char_array[idx] as char == '#' {
                point_vec.push(Point {
                    x: idx as i64,
                    y: line_count,
                })
            }
        }
        line_count += 1;
    }
    return (point_vec, width, height);
}

pub fn count_trees(map: &Vec<Point>, width: i64, height: i64, slope: Slope) -> i64{

    let mut p = Point { x: 0, y: 0 };

    let mut tree_count = 0;
    loop {
        if map.contains(&p) {
            tree_count += 1;
        }

        p.x += slope.run;
        p.x = p.x % width;
        p.y += slope.rise;

        if p.y >= height{
            break;
        }
    }
    return tree_count;
}

fn main() {
    let map_details = parse_map_to_vector("/home/jeremy/github/aoc_2020/day_3/input/input_1.txt");
    //let map_details = parse_map_to_vector("/home/jeremy/github/nic_aoc/data/raw/day3_input.txt");
    let map = map_details.0;
    let width = map_details.1 as i64;
    let height = map_details.2 as i64;

    let mut tree_count = 1;

    let slope  = Slope {rise: 1, run: 1};
    tree_count *= count_trees(&map, width, height, slope);

    let slope  = Slope {rise: 1, run: 3};
    tree_count *= count_trees(&map, width, height, slope);

    let slope  = Slope {rise: 1, run: 5};
    tree_count *= count_trees(&map, width, height, slope);

    let slope  = Slope {rise: 1, run: 7};
    tree_count *= count_trees(&map, width, height, slope);

    let slope  = Slope {rise: 2, run: 1};
    tree_count *= count_trees(&map, width, height, slope);

    println!("Counted {}", tree_count);
}
