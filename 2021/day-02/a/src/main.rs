use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-02.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut depth = 0;
    let mut horz = 0;
    input.trim().split('\n').for_each(|str| {
        let mut iter = str.trim().split_ascii_whitespace();
        let cmd = iter.next().unwrap();
        let x = iter.next().unwrap().parse::<i32>().unwrap();

        match cmd {
            "forward" => horz += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => {}
        };
    });

    fs::write("../../output/day-02-part-1.txt", (depth * horz).to_string().as_bytes())
        .expect("error when writing the answer.");
}
