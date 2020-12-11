use std::fs::File;
use std::io::{prelude::*, BufReader};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatType {
    Floor,
    Empty,
    Full,
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn parse_file(path: &str) -> Vec<Vec<SeatType>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut seat_map: Vec<Vec<SeatType>> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let mut row: Vec<SeatType> = Vec::new();

        for char in text.chars() {
            match char {
                '.' => row.push(SeatType::Floor),
                'L' => row.push(SeatType::Empty),
                '#' => row.push(SeatType::Full),
                _ => {
                    panic!("Char not found")
                }
            }
        }
        seat_map.push(row);
    }
    seat_map
}

fn print_map(seat_map: &Vec<Vec<SeatType>>) {
    for row in seat_map {
        for seat in row {
            match seat {
                SeatType::Floor => {
                    print!(".")
                }
                SeatType::Empty => {
                    print!("L")
                }
                SeatType::Full => {
                    print!("#")
                }
            }
        }
        println!("");
    }
}

fn count_adjacent(seat_map: &Vec<Vec<SeatType>>, seat: &Coordinate) -> (u32, u32) {
    let mut empty_count = 0;
    let mut full_count = 0;

    for direction in Direction::iter() {
        match check_direction(&seat_map, &seat, direction) {
            SeatType::Floor => { empty_count += 1; }
            SeatType::Empty => { empty_count += 1; }
            SeatType::Full => { full_count += 1; }
        }
    }
    return (empty_count, full_count);
}

fn check_direction(seat_map: &Vec<Vec<SeatType>>, seat: &Coordinate, dir: Direction) -> SeatType {
    match dir {
        Direction::North => {
            if seat.y == 0 {
                SeatType::Floor
            } else {
                seat_map[seat.y-1][seat.x]
            }
        }
        Direction::NorthEast => {
            if seat.y == 0 || seat.x >= seat_map[0].len() - 1 {
                SeatType::Floor
            } else {
                seat_map[seat.y - 1][seat.x + 1]
            }
        }
        Direction::East => {
            if seat.x >= seat_map[0].len() - 1 {
                SeatType::Floor
            } else {
                seat_map[seat.y][seat.x +1]
            }
        }
        Direction::SouthEast => {
            if seat.x >= seat_map[0].len() - 1 || seat.y >= seat_map.len() - 1 {
                SeatType::Floor
            } else {
                seat_map[seat.y + 1][seat.x+ 1]
            }
        }
        Direction::South => {
            if seat.y >= seat_map.len() - 1 {
                SeatType::Floor
            } else {
                seat_map[seat.y + 1][seat.x]
            }
        }
        Direction::SouthWest => {
            if seat.y >= seat_map.len() - 1 || seat.x == 0 {
                SeatType::Floor
            } else {
                seat_map[seat.y + 1][seat.x - 1]
            }
        }
        Direction::West => {
            if seat.x == 0 {
                SeatType::Floor
            } else {
                seat_map[seat.y][seat.x - 1]
            }
        }
        Direction::NorthWest => {
            if seat.x == 0 || seat.y == 0 {
                SeatType::Floor
            } else {
                seat_map[seat.y - 1][seat.x - 1]
            }
        }
    }
}

fn update_map(seat_map: &Vec<Vec<SeatType>>) -> Vec<Vec<SeatType>> {
    let mut new_map = Vec::new();
    for (y, row) in seat_map.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, _seat) in row.iter().enumerate() {
            let count = count_adjacent(seat_map, &Coordinate { x, y });
            if seat_map[y][x] == SeatType::Floor {
                new_row.push(SeatType::Floor);
            } else if seat_map[y][x] == SeatType::Empty && (count.0 == 8) {
                new_row.push(SeatType::Full);
            } else if count.1 >= 4 {
                new_row.push(SeatType::Empty);
            } else {
                new_row.push(seat_map[y][x])
            }
        }
        new_map.push(new_row);
    }
    new_map
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_direction() {
        let mut map = Vec::new();
        map.push(vec![SeatType::Full, SeatType::Empty, SeatType::Full]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);

        assert_eq!(check_direction(&map, &Coordinate { x: 0, y: 0 }, Direction::North), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 2, y: 0 }, Direction::NorthEast), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 2, y: 0 }, Direction::East), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 2, y: 2 }, Direction::SouthEast), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 0, y: 2 }, Direction::South), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 0, y: 2 }, Direction::SouthWest), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 0, y: 0 }, Direction::West), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 0, y: 0 }, Direction::NorthWest), SeatType::Floor);

        map = Vec::new();
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Floor]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);

        assert_eq!(check_direction(&map, &Coordinate { x: 1, y: 0 }, Direction::North), SeatType::Floor);
        assert_eq!(check_direction(&map, &Coordinate { x: 1, y: 0 }, Direction::West), SeatType::Full);
        assert_eq!(check_direction(&map, &Coordinate { x: 1, y: 0 }, Direction::SouthWest), SeatType::Full);
        assert_eq!(check_direction(&map, &Coordinate { x: 1, y: 0 }, Direction::South), SeatType::Full);


    }

    #[test]
    fn test_count_adjacent() {
        let mut map = Vec::new();
        map.push(vec![SeatType::Full, SeatType::Empty, SeatType::Full]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);

        //assert_eq!(count_adjacent(&map, &Coordinate { x: 0, y: 0 }), (6, 2));

        map = Vec::new();
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Floor]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);
        map.push(vec![SeatType::Full, SeatType::Full, SeatType::Full]);


        assert_eq!(count_adjacent(&map, &Coordinate { x: 1, y: 0 }), (4, 4));
    }
}

fn main() {
    let mut orig_map = parse_file("/home/jeremy/github/aoc_2020/day_11/input/input_1.txt");
    print_map(&orig_map);


    loop {
        let new_map = update_map(&orig_map);
        println!("");
        println!("");
        println!("");
        print_map(&new_map);
        if new_map == orig_map{
            break;
        }
        orig_map = new_map;
    }

    let mut full_count = 0;
    for (y, row) in orig_map.iter().enumerate(){
        for (x, _seat) in row.iter().enumerate(){
            if orig_map[y][x]==SeatType::Full{
                full_count +=1;
            }
        }
    }

    println!("Full Count: {}", full_count);
}

/*

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.
 */
