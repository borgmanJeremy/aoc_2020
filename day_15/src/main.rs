use ::std::collections::HashMap;

#[derive(Debug, Clone)]
struct Buff {
    n: i32,
    n_1: i32,
}

fn parse_file() -> Vec<i32> {
    let input_str = include_str!("../input/input.txt").split(",")
        .map(|x|
            x.parse::<i32>().unwrap()
        )
        .collect();
    input_str
}

fn main() {
    let input = parse_file();
    let mut turn_count = 1;
    let turn_limit = 30000000;

    let mut vec_history = Vec::new();

    for num in input {
        vec_history.push(num);
        //println!("{}, {}", turn_count, num);
        turn_count += 1;
    }


    let mut last_spoke = 0;
    vec_history.push(0);
    //println!("{}, {}", turn_count, 0);
    turn_count += 1;

    for idx in turn_count..turn_limit+1 {
        let mut iter = vec_history.iter();
        let most_recent = iter.rposition(|&x| x == last_spoke);
        let second_most_recent = iter.rposition(|&x| x == last_spoke);
        if second_most_recent.is_none() {
            let speak = 0;
            vec_history.push(speak as i32);
            last_spoke = speak as i32;
            //println!("{}, {}", turn_count, speak);
        } else {
            let speak = most_recent.unwrap() - second_most_recent.unwrap();
            vec_history.push(speak as i32);
            last_spoke = speak as i32;
            //println!("{}, {}", turn_count, speak);
        }
        turn_count += 1;
        //if turn_count % 10000 == 0{
        //    println!("{}%", (turn_count as f32 / turn_limit as f32)*100.);
        //}
    }

    println!("{}, {}", turn_count, last_spoke);
}
