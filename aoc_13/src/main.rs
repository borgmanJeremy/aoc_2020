fn parse_file() -> (i32, Vec<i32>) {
    let input_str = include_str!("../input/sample.txt");

    let input_list: Vec<&str> = input_str.split("\n").collect();
    let target = input_list[0].parse::<i32>().unwrap();

    let bus: Vec<i32> = input_list[1].split(",").filter(|&x| x.parse::<i32>().is_ok()).map(|x| x.parse::<i32>().unwrap()).collect();
    (target, bus)
}

fn part_1() {
    let (target, bus) = parse_file();

    let mut min = bus[0] - (target % bus[0]);
    let mut min_idx = 0;
    for (idx, elem) in bus.iter().enumerate() {
        let min_wait = elem - (target % elem);
        if min_wait < min {
            min = min_wait;
            min_idx = idx;
        }
        //println!("Target: {}, bus: {}, wait: {}", target, elem, min_wait);
    }
    println!("Min Wait : {}, Bus ID: {}, ans: {}", min, bus[min_idx], min * bus[min_idx]);
}

struct BusTime {
    id: u64,
    delay: u64,
}


fn part_2() {
    let input_str = include_str!("../input/input_1.txt");
    let input_list: Vec<&str> = input_str.split("\n").collect();
    let bus: Vec<_> = input_list[1].split(",")
        .map(|x| x.parse::<u64>())
        .map(
            |x| match x {
                Ok(val) => { Some(val) }
                Err(_) => { None }
            }).collect();

    let mut bus_array = Vec::new();
    for (idx, value) in bus.iter().enumerate() {
        if value.is_some() {
            bus_array.push(BusTime { id: value.unwrap() as u64, delay: idx as u64 })
        }
    }

    let mut time = 0;
    let mut delta = 1;
    for bus in bus_array {
        while (time + bus.delay) % bus.id != 0 {
            time += delta;
        }
        //Once match is found "lock in" delta like a gear
        delta *= bus.id;
    }

    println!("Time: {}", time);
}

fn main() {
    part_1();
    part_2();


    let input_str = include_str!("../input/input_1.txt");
    let input_list: Vec<&str> = input_str.split("\n").collect();

    let bus: Vec<_> = input_list[1].split(",")
        .map(|x| x.parse::<u64>())
        .map(
            |x| match x {
                Ok(val) => { Some(val) }
                Err(_) => { None }
            }).collect();


    let max = bus.iter().filter(|x| x.is_some()).max().unwrap().unwrap();
    let min = bus.iter().filter(|x| x.is_some()).min().unwrap().unwrap();
    let lcm = bus.iter().filter(|x| x.is_some()).fold(1, |acc, x| acc * x.unwrap());
}

