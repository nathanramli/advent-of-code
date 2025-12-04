use std::{
    collections::HashMap,
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

    let mut map = HashMap::new();

    input.trim().split('\n').for_each(|x| {
        let mut splits = x.split_ascii_whitespace().into_iter();
        let left = splits.next().unwrap().parse::<u32>().unwrap();
        let right = splits.next().unwrap().parse::<u32>().unwrap();

        lefts.push(left);
        rights.push(right);
    });

    rights.iter().for_each(|&x| {
        let incr_val = *&map.get(&x).unwrap_or(&0u32) + 1;
        map.insert(x, incr_val);
    });

    let len = lefts.len();
    for i in 0..len {
        let left = *lefts.get(i).unwrap();

        ans += left * map.get(&left).unwrap_or(&0u32);
    }

    fs::write("../../output/day-01-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
