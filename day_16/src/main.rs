use itertools::*;

#[derive(Debug, Clone)]
enum ParseState {
    Rules,
    YourTicket,
    OtherTicket,
}

#[derive(Debug, Clone)]
struct Bound {
    lower: i32,
    upper: i32,
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    rule_1: Bound,
    rule_2: Bound,
}

fn parse_rule(rule: &str) -> Rule {
    let name = rule.split(":").nth(0).unwrap();

    let rule_list: Vec<&str> = rule.split(":").nth(1).unwrap().split("or").collect();
    let rule_1: Vec<&str> = rule_list[0].split("-").collect();
    let rule_2: Vec<&str> = rule_list[1].split("-").collect();

    Rule {
        name: String::from(name),
        rule_1: Bound {
            lower: rule_1[0].trim().parse::<i32>().unwrap(),
            upper: rule_1[1].trim().parse::<i32>().unwrap(),
        },
        rule_2: Bound {
            lower: rule_2[0].trim().parse::<i32>().unwrap(),
            upper: rule_2[1].trim().parse::<i32>().unwrap(),
        },
    }
}

fn parse_ticket(ticket_string: &str) -> Vec<i32> {
    let ticket_list: Vec<i32> = ticket_string.split(",")
        .map(|x|
            x.parse::<i32>().unwrap()
        )
        .collect();
    ticket_list
}

fn parse_file() -> (Vec<Rule>, Vec<i32>, Vec<Vec<i32>>) {
    let input_str = include_str!("../input/input.txt");
    let input_lines: Vec<&str> = input_str.split("\n").collect();

    let mut rule_list = Vec::new();
    let mut your_ticket = Vec::new();
    let mut other_tickets = Vec::new();
    let mut state = ParseState::Rules;

    for line in input_lines {
        match &state {
            ParseState::Rules => {
                if line == "your ticket:" {
                    state = ParseState::YourTicket;
                } else if line.is_empty() {} else {
                    rule_list.push(parse_rule(&line));
                }
            }
            ParseState::YourTicket => {
                if line == "nearby tickets:" {
                    state = ParseState::OtherTicket;
                } else if line.is_empty() {} else {
                    your_ticket = parse_ticket(&line);
                }
            }

            ParseState::OtherTicket => {
                if line.is_empty() {} else {
                    let ticket = parse_ticket(&line);
                    other_tickets.push(ticket);
                }
            }
        }
    }
    (rule_list, your_ticket, other_tickets)
}

fn validate_ticket(ticket_num: i32, rule: &Rule) -> bool {
    if ticket_num >= rule.rule_1.lower && ticket_num <= rule.rule_1.upper {
        return true;
    } else if ticket_num >= rule.rule_2.lower && ticket_num <= rule.rule_2.upper {
        return true;
    }

    false
}

fn is_valid_ticket(ticket: &Vec<i32>, rule_list: &Vec<Rule>) -> bool {
    let count: Vec<&i32> = ticket.iter().filter(|num| {
        for rule in rule_list {
            if validate_ticket(**num, &rule) {
                return false;
                break;
            }
        }
        return true;
    }).collect();
    if count.len() >= 1 {
        return false;
    } else {
        return true;
    }
}

fn part_1() {
    let (rule_list, your_ticket, other_tickets) = parse_file();
    let mut error_count = 0;


    for ticket in &other_tickets {
        let count: Vec<&i32> = ticket.iter().filter(|num| {
            for rule in &rule_list {
                if validate_ticket(**num, &rule) {
                    return false;
                    break;
                }
            }
            return true;
        }).collect();
        error_count += count.iter().fold(0, |acc, num| acc + **num);
    }

    println!("{}", error_count);
}

fn check_whole_ticket(ticket: &Vec<i32>, rules: &Vec<&Rule>) -> bool {
    for (idx, num) in ticket.iter().enumerate() {
        if !validate_ticket(ticket[idx], &rules[idx]) {
            return false;
        }
    }
    true
}

fn main() {
    // part_1();


    let (rule_list, your_ticket, mut other_tickets) = parse_file();

    other_tickets = other_tickets.into_iter().filter(|x| is_valid_ticket(x, &rule_list)).collect();

    let mut cols = Vec::new();
    for idx in 0..other_tickets[0].len() {
        let col: Vec<i32> = other_tickets.iter().flatten().enumerate().filter(|&x| x.0 % other_tickets[0].len() == idx).map(|x| *x.1).collect();
        cols.push(col);
    }

    for (count,col) in cols.iter().enumerate() {
        println!("col: {}",count);
        for rule in &rule_list {
            if col.iter().filter(|&&x| validate_ticket(x, &rule)).count() == cols[0].len() {
                println!("{:?}", rule.name);
            }
        }
    }

    println!("Test");
}
