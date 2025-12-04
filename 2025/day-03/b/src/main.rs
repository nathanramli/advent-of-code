use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|line: &str| {
        let numbers = line.chars().map(|ch| ch.to_digit(10).unwrap() as u128).collect::<Vec<u128>>();

        let mut joltages = vec![];
        for i in (numbers.len() - 12)..numbers.len() {
            joltages.push(i);
        }


        for i in 0..joltages.len() {
            let joltage = joltages[i];
            
            let start= if i == 0 { 0 } else { joltages[i - 1] + 1 };
            let mut max = joltage;
            for k in start..joltage {
                // if bigger, move it
                if numbers[k] > numbers[max] {
                    max = k;
                // move it to left, if equal
                } else if numbers[k] == numbers[max] && k < max {
                    max = k;
                }
            }
            joltages[i] = max;
        }
        joltages.iter().enumerate().for_each(|(pos, &x)| ans += numbers[x] * 10u128.pow(11 - pos as u32));
    });

    fs::write("../../output/day-03-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
