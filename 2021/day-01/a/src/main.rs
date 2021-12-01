use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-01.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut i = 0;
    let mut last_number = 0;
    let mut ans = 0;
    input.trim().split('\n').for_each(|str| {
        let number = str.trim().parse::<i32>().unwrap();
        if i > 0 && number > last_number {
            ans += 1;
        }

        i += 1;
        last_number = number;
    });

    fs::write("../../output/day-01-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
