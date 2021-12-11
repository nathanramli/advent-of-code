use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-08.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let ans = input.trim().split('\n').fold(0, |sum, buff| {
        let after_delimiter = buff.split('|').skip(1).next().unwrap().trim();
        sum + after_delimiter
            .split_ascii_whitespace()
            .fold(0, |sum, buff| {
                if [2, 3, 4, 7].contains(&buff.len()) {
                    sum + 1
                } else {
                    sum
                }
            })
    });

    fs::write("../../output/day-08-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
