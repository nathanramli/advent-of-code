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

    input.trim().split('\n').for_each(|x| {
        let mut first = -1;
        let mut last = 0;
        x.chars().for_each(|x| {
            let x = x.to_string().parse::<i32>().unwrap_or(-1);

            if x != -1 {
                if first == -1 {
                    first = x;
                };
                last = x;
            }
        });
        ans += first * 10 + last;
    });

    fs::write("../../output/day-01-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
