use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-01.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut pos = 50;

    input.trim().split('\n').for_each(|line: &str| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<i32>().unwrap();
        let distance = distance % 100;

        if direction == 'L' {
            pos = pos - distance;
            if pos < 0 {
                pos += 100;
            }
        } else {
            pos = (pos + distance) % 100;
        }
        if pos == 0 {
            ans += 1;
        }
    });

    fs::write("../../output/day-01-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
