use std::fs;

fn do_operation(op: &str, operands: &Vec<i64>) -> i64 {
    match op {
        "+" => return operands.iter().sum(),
        "*" => return operands.iter().product(),
        _ => return i64::MIN,
    }
}

pub fn day6part2() {
    let input = fs::read_to_string("inputs/day6").expect("Cannot read file");
    
    let lines: Vec<&str> = input.lines().collect();

    let mut operands: Vec<i64> = Vec::new();

    let mut result = 0;

    for i in (0..lines[0].len()).rev()
    {
        let op: String = lines.iter().filter_map(|l| l.chars().nth(i).take_if(|c| !c.is_whitespace())).collect();

        if op.is_empty()
        {
            continue;
        }

        if "+*".contains(op.chars().last().unwrap())
        {
            let operator = op.chars().last().unwrap().to_string();
            operands.push(op[..(op.len() - 1)].parse::<i64>().unwrap());

            result += do_operation(&operator, &operands);

            operands.clear();
        }
        else {
            operands.push(op.parse::<i64>().unwrap());
        }
    }

    println!("{result}");
}

pub fn day6part1() {
    let input = fs::read_to_string("inputs/day6").expect("Cannot read file");

    let instructions: Vec<Vec<&str>> = input
        .lines()
        .map(|l: &str| l.split_whitespace().collect())
        .collect();

    let mut operands: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    for i in instructions {
        let mut row: Vec<i64> = Vec::new();

        for op in i {
            if let Some(parsed) = op.parse::<i64>().ok() {
                row.push(parsed);
            } else {
                operators.push(op);
            }
        }

        if !row.is_empty(){
            operands.push(row);
        }
    }

    let result: i64 = (0..operands[0].len())
        .map(|i| do_operation(operators[i], &operands.iter().map(|row| row[i]).collect()))
        .sum();

    println!("{result}");
}
