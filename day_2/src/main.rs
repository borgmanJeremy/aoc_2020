use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
struct PasswordReq {
    letter: char,
    min: u32,
    max: u32,
    password: String,
}

fn read_from_file(path: &str) -> Vec<PasswordReq> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut password_list = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let tokens: Vec<&str> = text.split(" ").collect();
                let bounds: Vec<&str> = tokens[0].split("-").collect();
                let letter = tokens[1].chars().nth(0).unwrap();
                let password = tokens[2];

                password_list.push(
                    PasswordReq {
                        letter,
                        min: bounds[0].parse::<u32>().unwrap(),
                        max: bounds[1].parse::<u32>().unwrap(),
                        password: String::from(password),
                    }
                );
            }
            Err(err) => { println!("Error reading line:{} ", err); }
        }
    }
    return password_list;
}

fn main() {
    let input_data = read_from_file("/home/jeremy/github/aoc_2020/day_2/input/input_1.txt");

    println!("Part 1");
    let mut valid_count = 0;
    for password in &input_data {
        let count = password.password.matches(password.letter).count() as u32;
        if (count <= password.max) && (count >= password.min) {
            valid_count += 1;
        }
    }
    println!("There are {} valid", valid_count);


    println!("Part 2");
    valid_count = 0;
    for password in input_data {
        let first_pos = password.password.as_bytes()[(password.min - 1) as usize] as char == password.letter;
        let last_pos = password.password.as_bytes()[(password.max - 1) as usize] as char == password.letter;

        if (first_pos || last_pos) && !(first_pos && last_pos) {
            valid_count += 1;
        }
    }

    println!("There are {} valid", valid_count);
}
