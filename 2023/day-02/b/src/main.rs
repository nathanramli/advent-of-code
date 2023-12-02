use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Clone, Copy)]
enum COLORS {
    RED,
    GREEN,
    BLUE,
    UNDEFINED,
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-02.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|line| {
        let mut line = line.split(": ");
        let line = line.nth(1).unwrap();
        let sets = line.split("; ").collect::<Vec<&str>>();

        let mut max_amount_color = [0; 3];

        sets.iter().for_each(|set| {
            set.split(", ").for_each(|set_color| {
                let mut set_color = set_color.split_ascii_whitespace();
                let amount = set_color.next().unwrap().parse::<i32>().unwrap();
                let color = set_color.next().unwrap();

                let idx = match color {
                    "red" => COLORS::RED,
                    "green" => COLORS::GREEN,
                    "blue" => COLORS::BLUE,
                    _ => COLORS::UNDEFINED,
                };

                max_amount_color[idx as usize] = max_amount_color[idx as usize].max(amount);
            });
        });

        ans += max_amount_color.iter().product::<i32>();
    });

    fs::write("../../output/day-02-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
