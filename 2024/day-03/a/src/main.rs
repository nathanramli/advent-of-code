use std::{
    fs::{self, File},
    io::Read,
};

use regex::Regex;

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|row| {
        let row = row.trim();
        
        let regex = Regex::new(r"(mul\(\d{1,3}\,\d{1,3}\))").unwrap();
        let matches = regex.find_iter(row).map(|m| m.as_str()).collect::<Vec<_>>();

        matches.iter().for_each(|&formula| {
            let regex = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
            let caps = regex.captures(formula).unwrap();
            let a = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let b = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

            ans += a * b;
        });
    });

    fs::write("../../output/day-03-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
