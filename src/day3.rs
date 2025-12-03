use std::cmp::Reverse;
use std::fs;

pub fn day3() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("Cannot read file");
    let banks = contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mut sum: u64 = 0;

    for bank in banks {
        let nums: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut i = 0;
        let mut indices: [usize; 12] = [0; 12];

        let mut start = 0;
        let mut end = nums.len() - 12 + i;

        let mut result = 0;

        while i < 12 {
            let sl;

            sl = &nums[start..=end];

            let (index, &max_val) = sl
                .iter()
                .enumerate()
                .min_by_key(|&(_, &v)| Reverse(v))
                .unwrap();
            indices[i] = start + index;

            i += 1;
            start = start + index + 1;
            end = nums.len() - 12 + i;

            result += (max_val as u64) * 10_u64.pow((12 - i) as u32);
        }

        sum += result;
    }

    println!("{sum}");
}
