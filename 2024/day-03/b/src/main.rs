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
    let mut ok = true;

    input.trim().split('\n').for_each(|row| {
        let row = row.trim();
        
        let regex = Regex::new(r"(mul\(\d{1,3}\,\d{1,3}\)|do\(\)|don't\(\))").unwrap();
        let matches = regex.find_iter(row).map(|m| m.as_str()).collect::<Vec<_>>();

        matches.iter().for_each(|formula| {            
            let regex = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
            let caps = regex.captures(formula);
            if caps.is_none() {
                if formula.eq(&"do()") {
                    ok = true;
                } else {
                    ok = false;
                }
            } else {
                let caps = caps.unwrap();
                let a = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let b = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
                if ok {
                    ans += a * b;
                }
            }
        });
    });

    fs::write("../../output/day-03-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}

