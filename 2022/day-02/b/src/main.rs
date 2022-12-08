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
    input.trim().split('\n').for_each(|row| {
        let mut input_row = row.clone().trim().split_whitespace();
        let col1 = input_row.next().unwrap();
        let mut col2 = input_row.next().unwrap();

        if col2 == "X" {
            if col1 == "A" {
                col2 = "Z";
            } else if col1 == "B" {
                col2 = "X";
            } else {
                col2 = "Y";
            }
        } else if col2 == "Y" {
            if col1 == "A" {
                col2 = "X";
            } else if col1 == "B" {
                col2 = "Y";
            } else {
                col2 = "Z";
            }
        } else {
            if col1 == "A" {
                col2 = "Y";
            } else if col1 == "B" {
                col2 = "Z";
            } else {
                col2 = "X";
            }
        }

        let mut score = 0;
        match col2 {
            "X" => {
                score += 1;

                if col1 == "C" {
                    score += 6;
                } else if col1 == "A" {
                    score += 3;
                }
            }
            "Y" => {
                score += 2;

                if col1 == "A" {
                    score += 6;
                } else if col1 == "B" {
                    score += 3;
                }
            }
            "Z" => {
                score += 3;

                if col1 == "B" {
                    score += 6;
                } else if col1 == "C" {
                    score += 3;
                }
            }
            _ => println!("No"),
        };
        ans += score;
    });

    fs::write("../../output/day-02-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
