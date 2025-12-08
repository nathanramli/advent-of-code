use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-06.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut numbers = vec![vec![]];
    let mut symbol = vec![];

    input.trim().split('\n').for_each(|line: &str| {
        let splits = line.split_ascii_whitespace();
        let mut i = 0;
        splits.for_each(|x| {
            let ch = x.chars().next().unwrap();
            if ch.is_numeric() {
                if i == numbers.len() {
                    numbers.push(vec![]);
                }
                numbers[i].push(x.parse::<u64>().unwrap());
                i += 1;
            } else {
                symbol.push(ch);
            };
        });
    });

    let mut i = 0;
    numbers.iter().for_each(|vec| {
        let mut sum = 0;
        for &num in vec {
            if symbol[i] == '+' {
                sum += num;
            } else {
                if sum == 0 {
                    sum = num;
                } else {
                    sum *= num;
                }
            }
        }
        ans += sum;
        i += 1;
    });

    fs::write("../../output/day-06-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
