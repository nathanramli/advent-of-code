//! Solution A and B is the same
//! They're already optimized with DP

use std::{
    fs::{self, File},
    io::Read,
};

const N: i64 = 256;

fn calculate(x: i64, mut remaining_day: i64, mem: &mut Vec<i64>) -> i64 {
    if x + 1 > remaining_day {
        return 0;
    }

    remaining_day -= x + 1;

    let mut sum = 1 + {
        if mem[remaining_day as usize] != -1 {
            mem[remaining_day as usize]
        } else {
            let ans = calculate(8, remaining_day, mem);
            mem[remaining_day as usize] = ans;
            ans
        }
    };

    while remaining_day - 7 >= 0 {
        remaining_day -= 7;
        sum += 1 + {
            if mem[remaining_day as usize] != -1 {
                mem[remaining_day as usize]
            } else {
                let ans = calculate(8, remaining_day, mem);
                mem[remaining_day as usize] = ans;
                ans
            }
        };
    }
    sum
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-06.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut mem = vec![-1i64; (N + 1) as usize];
    let mut sum = 0;

    input.trim().split(',').for_each(|str| {
        sum += 1 + calculate(str.parse::<i64>().unwrap(), N, &mut mem);
    });

    fs::write("../../output/day-06-part-2.txt", sum.to_string().as_bytes())
        .expect("error when writing the answer.");
}
