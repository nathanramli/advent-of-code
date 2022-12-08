use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-01.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut cons = 0;
    let mut totals = vec![];
    input.trim().split('\n').for_each(|x| {
        if x != "" {
            let number = x.trim().parse::<i32>().unwrap();
            cons += number;
        } else {
            totals.push(cons);
            cons = 0;
        }
    });
    totals.push(cons);

    totals.sort();

    fs::write(
        "../../output/day-01-part-2.txt",
        totals
            .iter()
            .rev()
            .take(3)
            .sum::<i32>()
            .to_string()
            .as_bytes(),
    )
    .expect("error when writing the answer.");
}
