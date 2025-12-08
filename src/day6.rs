use std::fs;

fn do_operation(op: &str, operands: Vec<i64>) -> i64 {
    match op {
        "+" => return operands.iter().sum(),
        "*" => return operands.iter().product(),
        _ => return i64::MIN,
    }
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
        .map(|i| do_operation(operators[i], operands.iter().map(|row| row[i]).collect()))
        .sum();

    println!("{result}");
}
