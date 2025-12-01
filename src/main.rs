use std::fs;

fn turn_left(current: i32, steps: i32) -> i32 {
    return (current + steps).rem_euclid(100);
}

fn turn_right(current: i32, steps: i32) -> i32 {
    return (current - steps).rem_euclid(100);
}

fn main() {
    let a = 0..=99;
    println!("List: {:?}", a.collect::<Vec<i32>>());

    let contents = fs::read_to_string("inputs/day1.txt").expect("Cannot read file");

    println!("Input: {contents}");

    let mut dial: i32 = 50;
    let mut zeros = 0;

    let instructions = contents.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty());

    for i in instructions {
        if i.chars().nth(0) == Some('L') {
            dial = turn_left(dial, i[1..].parse::<i32>().unwrap());
        } else {
            dial = turn_right(dial, i[1..].parse::<i32>().unwrap());
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    println!("Password: {zeros}");
}
