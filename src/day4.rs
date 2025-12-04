use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<Vec<char>> {
    let r = BufReader::new(File::open("inputs/day4.txt").unwrap());
    return r
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect();
}

fn check_around(x: i32, y: i32, input: &[Vec<char>]) -> i32 {
    let mut neighbours = 0;

    for j in (y - 1)..=(y + 1) {
        if j < 0 || j > (input.len() - 1) as i32 {
            continue;
        }

        for i in (x - 1)..=(x + 1) {
            if i < 0 || i > (input.len() - 1) as i32 || (i == x) && (j == y) {
                continue;
            }

            if input[j as usize][i as usize] == '@' {
                neighbours += 1;
            }
        }
    }

    return neighbours;
}

fn part1(input: Vec<Vec<char>>) {
    let w = input[0].len();
    let h = input.len();

    let mut accessibles = 0;

    for y in 0..h {
        for x in 0..w {
            if input[y][x] == '@' && check_around(x as i32, y as i32, &input) < 4 {
                accessibles += 1
            }
        }
    }

    println!("{accessibles}");
}

fn part2(mut input: Vec<Vec<char>>) {
        let w = input[0].len();
    let h = input.len();

    let mut removed = 0;
    let mut accessible = i32::MAX;

    while accessible > 0
    {
        accessible = 0;

        for y in 0..h {
            for x in 0..w {
                if input[y][x] == '@' && check_around(x as i32, y as i32, &input) < 4 {
                    input[y][x] = '.';
                    accessible += 1;
                    removed += 1;
                }
            }
        }
    }


    println!("{removed}");
}

pub fn day4() {
    let input = read_input();
    // part1(input);
    part2(input);
}
