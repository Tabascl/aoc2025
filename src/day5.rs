use std::collections::HashSet;
use std::fs;

pub fn day5part2() {
    let contents = fs::read_to_string("inputs/day5.txt").expect("Cannot read file");
    let (part1, _) = contents.split_once("\n\n").unwrap();

    let mut ranges: Vec<(i64, i64)> = part1
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();

            let start = a.parse::<i64>().unwrap();
            let end = b.parse::<i64>().unwrap();

            return (start, end);
        })
        .collect();

    ranges.sort();

    let mut merged: Vec<(i64, i64)> = Vec::new();

    for (start, end) in ranges {
        if let Some((_, prev_end)) = merged.last_mut() {
            if start <= *prev_end + 1 {
                *prev_end = (*prev_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let total: i64 = merged.iter().map(|(s, e)| e - s + 1).sum();

    println!("{total}");
}

pub fn day5() {
    let contents = fs::read_to_string("inputs/day5.txt").expect("Cannot read file");
    let (part1, part2) = contents.split_once("\n\n").unwrap();

    let ranges: Vec<_> = part1
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();

            let start = a.parse::<i64>().unwrap();
            let end = b.parse::<i64>().unwrap();

            return start..=end;
        })
        .collect();

    let ids = part2
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap());

    let mut valids = 0;

    for id in ids {
        ranges.iter().any(|r| r.contains(&id)).then(|| valids += 1);
    }

    println!("{valids}");
}
