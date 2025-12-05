use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-05.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut checks = vec![];
    let mut ranges = vec![];


    input.trim().split('\n').for_each(|line: &str| {
        if !line.is_empty() {
            if line.find('-').is_none() {
                checks.push(line.parse::<u64>().unwrap());
            } else {
                let parts= line.split('-').collect::<Vec<_>>();
                let start = parts[0].parse::<u64>().unwrap();
                let end = parts[1].parse::<u64>().unwrap();

                ranges.push((start, end));
            }
        }
    });

    for check in checks {
        for (start, end) in &ranges {
            if check >= *start && check <= *end {
                ans += 1;
                break;
            }
        }
    }

    fs::write("../../output/day-05-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
