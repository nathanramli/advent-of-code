use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-02.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|line: &str| {
        let ranges = line
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        
        let range = ranges.iter().map(|f| {
            let mut iter = f.split('-');
            let begin = iter.next().unwrap().parse::<u64>().unwrap();
            let end = iter.next().unwrap().parse::<u64>().unwrap();

            (begin, end)
        }).collect::<Vec<(u64, u64)>>();

        range.iter().for_each(|v| {
            let first = v.0;
            let end = v.1;

            for number in first..=end {
                let str = number.to_string();
                if str.len() % 2 == 0 {
                    let mid = str.len() / 2;
                    let (left, right) = str.split_at(mid);
                    if left == right {
                        ans += number;
                    }
                }
            }
        });
    });

    fs::write("../../output/day-02-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
