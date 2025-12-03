use std::fs;

pub fn day2() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("Cannot read file");
    let ranges = contents
        .split_terminator(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mut sum: i128 = 0;

    for r in ranges {
        let (start, end): (i64, i64) = r
            .split_once('-')
            .and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
            .unwrap_or_else(|| panic!("Failed to parse range from: {:?}", r));

        for id in start..=end {
            let sid = id.to_string();
            let mut slice = sid.len() / 2;

            while slice > 0 {
                let substr = &sid[..slice];

                if substr.repeat(sid.len().div_ceil(slice)) == sid {
                    sum += i128::from(id);
                    break;
                } else {
                    slice -= 1;
                }
            }
        }
    }

    println!("{sum}");
}
