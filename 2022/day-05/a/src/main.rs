use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-05.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut crates: Vec<Vec<char>> = vec![];
    let mut on_input_crates = true;
    input.split('\n').for_each(|row| {
        let mut i = 0i32;

        if on_input_crates {
            row.chars().for_each(|x| {
                i += 1;
                if x.is_ascii_digit() {
                    on_input_crates = false;

                    for i in 0..crates.len() {
                        crates[i].reverse();
                    }
                }

                if on_input_crates && (x.is_uppercase() || x.is_lowercase()) {
                    let at = (i as f64 / 4f64).ceil() as usize;
                    while crates.len() < at {
                        crates.push(vec![]);
                    }

                    crates[at - 1].push(x);
                }
            });
        } else {
            if row != "" {
                let mut words = row.split_ascii_whitespace().into_iter();
                words.next();
                let n = words.next().unwrap().parse::<i32>().unwrap();
                words.next();
                let from = words.next().unwrap().parse::<i32>().unwrap();
                words.next();
                let to = words.next().unwrap().parse::<i32>().unwrap();

                for _ in 0..n {
                    let x = crates[from as usize - 1].pop().unwrap();
                    crates[to as usize - 1].push(x);
                }
            }
        }
    });

    let ans = crates
        .iter()
        .fold(vec![], |mut acc, x| {
            acc.push(x.last().copied().unwrap());
            acc
        })
        .iter()
        .collect::<String>();
    fs::write("../../output/day-05-part-1.txt", ans.as_bytes())
        .expect("error when writing the answer.");
}
