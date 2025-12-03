use std::fs;

pub fn day3() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("Cannot read file");
    let banks = contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mut sum: u32 = 0;

    for bank in banks {
        let nums: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut first = 0;
        let mut second = 1;

        let mut i = 0;
        while i < nums.len() {
            if nums[i] > nums[first] && i < nums.len() - 1 {
                first = i;
                second = i + 1;
            }

            if nums[i] > nums[second] && i > first {
                second = i;
            }

            i += 1;
        }

        sum += nums[first] * 10 + nums[second];
    }

    println!("{sum}");
}
