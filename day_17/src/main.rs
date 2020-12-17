#![feature(map_into_keys_values)]

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
struct Map {
    map: HashMap<Coordinate, Status>
}


impl Map {
    fn new() -> Map {
        Map { map: HashMap::new() }
    }

    fn print_map(&self) {
        println!("");
        let min_x = self.map.clone().into_keys().map(|x| x.x).min().unwrap();
        let max_x = self.map.clone().into_keys().map(|x| x.x).max().unwrap();

        let min_y = self.map.clone().into_keys().map(|x| x.y).min().unwrap();
        let max_y = self.map.clone().into_keys().map(|x| x.y).max().unwrap();

        let min_z = self.map.clone().into_keys().map(|x| x.z).min().unwrap();
        let max_z = self.map.clone().into_keys().map(|x| x.z).max().unwrap();

        for z in min_z..max_z + 1 {
            println!("z={}", z);
            for y in (min_y..max_y + 1).rev() {
                for x in min_x..max_x + 1 {
                    if !self.map.contains_key(&Coordinate { x, y, z }) {
                        print!("!");
                    } else {
                        match (self.map[&Coordinate { x, y, z }])
                        {
                            Status::Active => { print!("#") }
                            Status::Inactive => { print!(".") }
                        }
                    }
                }
                println!("");
            }
        }
    }
    fn add_point(&mut self, coordinate: Coordinate, status: Status) {
        self.map.insert(coordinate.clone(), status);

        for z in -1..2 {
            for y in -1..2 {
                for x in -1..2 {
                    if !(x == 0 && y == 0 && z == 0) && !self.map.contains_key(&Coordinate {
                        x: x + coordinate.x,
                        y: y + coordinate.y,
                        z: z + coordinate.z,
                    }) {
                        self.map.insert(Coordinate {
                            x: x + coordinate.x,
                            y: y + coordinate.y,
                            z: z + coordinate.z,
                        }, Status::Inactive);
                    }
                }
            }
        }
    }

    fn count_active_neighbors(&self, coordinate: &Coordinate) -> i32 {
        let mut coord_list = Vec::new();

        for z in -1..2 {
            for y in -1..2 {
                for x in -1..2 {
                    if !(x == 0 && y == 0 && z == 0) {
                        coord_list.push(
                            Coordinate {
                                x: x + coordinate.x,
                                y: y + coordinate.y,
                                z: z + coordinate.z,
                            });
                    }
                }
            }
        }

        let mut count = 0;
        for coord in coord_list {
            if self.map.contains_key(&coord) {
                if self.map[&coord] == Status::Active {
                    count += 1;
                }
            }
        }
        count
    }
}

fn parse_map() -> Map {
    let map_str = include_str!("../input/input_1.txt");
    let mut y = 0;
    let mut map = Map::new();
    for line in map_str.lines() {
        let mut x = 0;
        for char in line.chars() {
            match char {
                '#' => { map.add_point(Coordinate { x, y, z: 0 }, Status::Active) }
                '.' => { map.add_point(Coordinate { x, y, z: 0 }, Status::Inactive) }
                _ => { panic!("Not recognized") }
            }
            x += 1;
        }
        y -= 1;
    }
    map
}


fn main() {
    //let mut map = Map::new();
    let mut map = parse_map();

    map.print_map();
    for _i in 0..6 {
        let mut new_map = map.clone();
        for point in map.map.iter() {
            let count = map.count_active_neighbors(&point.0);
            match &point.1 {
                Status::Active => {
                    if !(count == 2 || count == 3) {
                        new_map.add_point(point.0.clone(), Status::Inactive);
                    }
                }

                Status::Inactive => {
                    if count == 3 {
                        new_map.add_point(point.0.clone(), Status::Active);
                    }
                }
            }
        }
        map = new_map;
        //map.print_map();
    }
    let final_count = map.map.iter().filter(|&x| *x.1 == Status::Active).count();
    println!("final count: {}", final_count);
}
