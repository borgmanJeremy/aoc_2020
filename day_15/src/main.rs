use ::std::collections::HashMap;

#[derive(Debug, Clone)]
struct Buff {
    n: i32,
    n_1: Option<i32>,
}

fn parse_file() -> Vec<i32> {
    let input_str = include_str!("../input/input.txt").split(",")
        .map(|x|
            x.parse::<i32>().unwrap()
        )
        .collect();
    input_str
}

fn what_to_say(history: &HashMap<i32, Buff>, last_said: i32, was_first_said: bool) -> i32 {
    if was_first_said {
        0
    } else {
        let buff = history[&last_said].clone();
        buff.n - buff.n_1.unwrap()
    }
}

fn say(history: &mut HashMap<i32, Buff>, value: i32, turn_number: i32) -> bool {
    if history.contains_key(&value) {
        let old_turn_number = history[&value].n;
        let new_buff = Buff { n: turn_number, n_1: Some(old_turn_number) };
        history.insert(value, new_buff);
        //println!("{} {}", turn_number, value);
        false
    } else {
        history.insert(value, Buff { n: turn_number, n_1: None });
        //println!("{} {}", turn_number, value);
        true
    }
}

fn main() {
    let input = parse_file();
    let turn_limit = 30000000;

    let mut history = HashMap::new();


    let mut was_first = false;
    for (turn, val) in input.iter().enumerate() {
        was_first = say(&mut history, *val, turn as i32 + 1);
    }

    let mut said = 0;

    for turn in (input.len() as i32 + 1)..turn_limit + 1 {
        said = what_to_say(&history, said, was_first);
        was_first = say(&mut history, said, turn);
        if turn % 10000 == 0 {
            println!("{}%", (turn as f32 / turn_limit as f32) * 100.);
        }
    }
    println!("{:?}", said);
}
