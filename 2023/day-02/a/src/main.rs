use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-02.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|line| {
        let mut line = line.split(": ");
        let id = line
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let line = line.next().unwrap();
        let sets = line.split("; ").collect::<Vec<&str>>();
        let mut ok = true;

        sets.iter().for_each(|set| {
            set.split(", ").for_each(|set_color| {
                let mut set_color = set_color.split_ascii_whitespace();
                let amount = set_color.next().unwrap().parse::<i32>().unwrap();
                let color = set_color.next().unwrap();

                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => 0,
                };

                if amount > max {
                    ok = false;
                };
            });
        });

        if ok {
            ans += id;
        };
    });

    fs::write("../../output/day-02-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
