use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|line: &str| {
        let numbers = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let mut max_left = numbers[0];
        let mut left = 0;

        let mut max_right = numbers[1];
        let mut right = 1;

        for i in 1..numbers.len() {
            let number = numbers[i];

            if number > max_left && i != numbers.len() - 1 {
                max_left = number;
                left = i;

                max_right = numbers[i + 1];
                right = i + 1;
            }

            if number > max_right && i != right && i != left {
                max_right = number;
                right = i;
            }
        }

        ans += max_left * 10 + max_right;
    });

    fs::write("../../output/day-03-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
