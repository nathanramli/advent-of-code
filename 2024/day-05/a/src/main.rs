use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-05.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut rules = vec![];
    input.trim().split('\n').for_each(|row| {
        if !row.is_empty() {
            if row.find('|').is_some() {
                let splits = row.split('|').collect::<Vec<&str>>();
                rules.push((
                    splits[0].parse::<i64>().unwrap(),
                    splits[1].parse::<i64>().unwrap(),
                ));
            } else {
                let seq = row.split(",").map(|x| {
                    x.parse::<i64>().unwrap()
                }).collect::<Vec<i64>>();

                let mut ok = true;
                rules.iter().for_each(|(left, right)| {
                    let a = seq.iter().position(|&el| el == *right);
                    if a.is_none() {
                        return
                    }

                    let b = seq.iter().skip(a.unwrap()).position(|&el| el == *left);
                    if b.is_none() {
                        return
                    }

                    ok = false;
                });

                if ok {
                    ans += seq[seq.len() / 2];
                }
            }
        }
    });

    fs::write("../../output/day-05-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
