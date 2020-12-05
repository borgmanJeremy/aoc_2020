use std::fs::File;
use std::io::{prelude::*, BufReader};
use passport::{*};

#[derive(Debug, Clone)]
struct Passport {
    birth: Option<String>,
    issue: Option<String>,
    expr: Option<String>,
    height: Option<String>,
    eye: Option<String>,
    hair: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn has_valid_fields(passport: &Passport) -> bool {
    if passport.birth.is_some() &&
        passport.issue.is_some() &&
        passport.expr.is_some() &&
        passport.height.is_some() &&
        passport.eye.is_some() &&
        passport.hair.is_some() &&
        passport.pid.is_some() {
        true
    } else {
        false
    }
}

fn parse_to_passport(input: &Vec<String>) -> Passport {
    let mut passport = Passport {
        birth: None,
        issue: None,
        expr: None,
        height: None,
        eye: None,
        hair: None,
        pid: None,
        cid: None,
    };

    for token in input {
        let split: Vec<&str> = token.split(":").collect();
        match split[0] {
            "byr" => { passport.birth = Some(String::from(split[1])); }
            "iyr" => { passport.issue = Some(String::from(split[1])); }
            "eyr" => { passport.expr = Some(String::from(split[1])); }
            "hgt" => { passport.height = Some(String::from(split[1])); }
            "hcl" => { passport.hair = Some(String::from(split[1])); }
            "ecl" => { passport.eye = Some(String::from(split[1])); }
            "pid" => { passport.pid = Some(String::from(split[1])); }
            "cid" => { passport.cid = Some(String::from(split[1])); }
            _ => { panic!("identifier not found"); }
        }
    }
    passport
}

fn read_from_file(path: &str) -> Vec<Passport> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut passport_list = Vec::new();

    let mut token_list: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => {
                if text.is_empty() {
                    let passport = parse_to_passport(&token_list);
                    passport_list.push(passport);
                    token_list.clear();
                } else {
                    let t_list: Vec<&str> = text.split(" ").collect();
                    for token in t_list {
                        token_list.push(String::from(token));
                    }
                }
            }
            Err(err) => { println!("Error reading line:{} ", err); }
        }
    }
    return passport_list;
}


fn main() {
    let passport_list = read_from_file("/home/jeremy/github/aoc_2020/day_4/input/input_1.txt");

    let mut good_count = 0;
    let mut bad_count = 0;
    for passport in &passport_list {
        if has_valid_fields(&passport) {
            good_count += 1;
        } else {
            bad_count += 1;
        }
    }
    println!("Good Count is: {}, Bad Count is: {}", good_count, bad_count);

    good_count = 0;
    for passport in &passport_list {
        if has_valid_fields(&passport) {
            if validate_birth(passport.birth.as_ref().unwrap()) &&
                validate_issue(passport.issue.as_ref().unwrap()) &&
                validate_expr(passport.expr.as_ref().unwrap()) &&
                validate_height(passport.height.as_ref().unwrap()) &&
                validate_hair(passport.hair.as_ref().unwrap()) &&
                validate_eye(passport.eye.as_ref().unwrap()) &&
                validate_pid(passport.pid.as_ref().unwrap())
            {
                good_count += 1;
            }
        }
    }


    println!("Good Count is: {}", good_count);
}
