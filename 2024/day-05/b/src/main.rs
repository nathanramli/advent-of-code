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
                let mut seq = row
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                let mut is_ordered = true;
                rules.iter().for_each(|(left, right)| {
                    let a = seq.iter().position(|&el| el == *right);
                    if a.is_none() {
                        return;
                    }

                    let b = seq.iter().skip(a.unwrap()).position(|&el| el == *left);
                    if b.is_none() {
                        return;
                    }

                    is_ordered = false;
                });

                if is_ordered == false {
                    let mut relevant_rules = vec![];

                    rules.iter().for_each(|(left, right)| {
                        if seq.iter().find(|&x| x == left).is_some()
                            && seq.iter().find(|&x| x == right).is_some()
                        {
                            relevant_rules.push((*left, *right));
                        }
                    });

                    let mut new_seq = vec![];
                    while !seq.is_empty() {
                        seq.iter().for_each(|x| {
                            let found = relevant_rules.iter().find(|pair| *x == pair.1).is_some();
                            if !found {
                                new_seq.push(*x);
                            }
                        });
                        seq = seq
                            .iter()
                            .filter(|&x| new_seq.iter().find(|&y| *y == *x).is_none())
                            .cloned()
                            .collect();

                        relevant_rules = relevant_rules
                            .iter()
                            .filter(|&pair| {
                                new_seq
                                    .iter()
                                    .find(|&y| *y == pair.0 || *y == pair.1)
                                    .is_none()
                            })
                            .cloned()
                            .collect();
                    }

                    ans += new_seq[new_seq.len() / 2];
                }
            }
        }
    });

    fs::write("../../output/day-05-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
