fn parse_file() -> (i32, Vec<i32>) {
    let input_str = include_str!("../input/sample.txt");

    let input_list: Vec<&str> = input_str.split("\n").collect();
    let target = input_list[0].parse::<i32>().unwrap();

    let bus: Vec<i32> = input_list[1].split(",").filter(|&x| x.parse::<i32>().is_ok()).map(|x| x.parse::<i32>().unwrap()).collect();
    (target, bus)
}

fn part_1(){
    let (target, bus) = parse_file();

    let mut min = bus[0] - (target % bus[0]);
    let mut min_idx = 0;
    for (idx, elem) in bus.iter().enumerate() {
        let min_wait = elem - (target % elem);
        if min_wait < min{
            min = min_wait;
            min_idx = idx;
        }
        //println!("Target: {}, bus: {}, wait: {}", target, elem, min_wait);
    }
    println!("Min Wait : {}, Bus ID: {}, ans: {}", min, bus[min_idx], min * bus[min_idx]);
}



fn main() {
    //part_1();


    let (target, bus) = parse_file();
}

