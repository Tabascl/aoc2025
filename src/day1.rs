use std::fs;

pub fn day1()
{
        let contents = fs::read_to_string("inputs/day1.txt").expect("Cannot read file");

    let mut dial: i32 = 50;
    let mut zeros = 0;

    let instructions = contents
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    for i in instructions {
        let steps = i[1..].parse::<i32>().unwrap();

        let res;
        if i.chars().nth(0) == Some('L') {
            res = dial - steps;
        } else {
            res = dial + steps;
        }

        zeros += (res / 100).abs();

        if (res <= 0) && (dial != 0) {
            zeros += 1;
        }

        dial = res.rem_euclid(100);
    }

    println!("Password: {zeros}");
}