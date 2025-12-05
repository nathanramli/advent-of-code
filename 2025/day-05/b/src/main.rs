use std::{
    fs::{self, File},
    io::Read,
};

fn find_closest_start(ranges: &Vec<(u64, u64)>, x: u64) -> Option<(u64, u64)> {
    let mut candidate: Option<(u64, u64)> = None;

    for (start, end) in ranges {
        if *start > x {
            match candidate {
                Some((cand_start, _)) => {
                    if cand_start - x > *start - x {
                        candidate = Some((*start, *end));
                    }
                }
                None => {
                    candidate = Some((*start, *end));
                }
            }
        }
    }

    candidate
}

fn find_range_within_x_with_highest_end(
    ranges: &Vec<(u64, u64)>,
    x: u64,
) -> Option<(u64, u64)> {
    let mut candidate: Option<(u64, u64)> = None;

    for (start, end) in ranges {
        if *start <= x && *end >= x {
            match candidate {
                Some((_, cand_end)) => {
                    // 6.
                    if cand_end < *end {
                        candidate = Some((*start, *end));
                    }
                }
                None => {
                    candidate = Some((*start, *end));
                }
            }
        }
    }

    candidate
}


// 1. sort
// 2. start to find the smallest number, from the current number
// 3. end at the range of that smallest number
// 4. find range that includes the end number + 1
// 5. if there are multiples, find with the largest end number
// 6. repeat until no more ranges can be found
// 7. back to step 2
fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-05.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut ranges = vec![];


    input.trim().split('\n').for_each(|line: &str| {
        if !line.is_empty() {
            if !line.find('-').is_none() {
                let parts= line.split('-').collect::<Vec<_>>();
                let start = parts[0].parse::<u64>().unwrap();
                let end = parts[1].parse::<u64>().unwrap();

                ranges.push((start, end));
            }
        }
    });

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut max = 0;
    ranges.iter().for_each(|x| {
        max = max.max(x.1);
    });

    let mut current = 0;
    while current <= max {
        // 2.
        let range = find_closest_start(&ranges, current);
        if range.is_none() {
            break;
        }
        let range = range.unwrap();

        // 3.
        ans += range.1 - range.0 + 1;

        // 4.
        current = range.1 + 1;
        loop {
            let range = find_range_within_x_with_highest_end(&ranges, current);
            if range.is_none() {
                break;
            }
            let range = range.unwrap();
            ans += range.1 - current + 1;
            current = range.1 + 1;
        }
        
    }

    fs::write("../../output/day-05-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
