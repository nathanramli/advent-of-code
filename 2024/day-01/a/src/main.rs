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

    let mut lefts = vec![];
    let mut rights = vec![];

    input.trim().split('\n').for_each(|x| {
        let mut splits = x.split_ascii_whitespace().into_iter();
        let left = splits.next().unwrap().parse::<u32>().unwrap();
        let right = splits.next().unwrap().parse::<u32>().unwrap();

        lefts.push(left);
        rights.push(right);
    });

    lefts.sort();
    rights.sort();

    let len = lefts.len();
    for i in 0..len {
        let left = *lefts.get(i).unwrap();
        let right = *rights.get(i).unwrap();

        ans += left.abs_diff(right);
    }

    fs::write("../../output/day-01-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
