use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-01.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut last_sum = 0;
    let mut i = 0;
    let mut ans = 0;

    input
    .trim()
    .split('\n')
    .map(|s| {
        s.trim().parse::<i32>().unwrap()
    })
    .collect::<Vec<i32>>()
    .windows(3)
    .for_each(|str| {
        let curr = str.iter().sum();

        if i > 0 && curr > last_sum {
            ans += 1;
        }

        i += 1;
        last_sum = curr;
    });

    fs::write("../../output/day-01-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
