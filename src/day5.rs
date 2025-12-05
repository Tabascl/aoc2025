use std::fs;

pub fn day5()
{
    let contents = fs::read_to_string("inputs/day5.txt").expect("Cannot read file");
    let (ranges, ids) = contents.split_once("\n\n").unwrap();

    
}