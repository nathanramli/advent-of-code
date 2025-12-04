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
        let splits = row.split_ascii_whitespace().into_iter();
        let row = splits.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut is_incr = true;
        let mut is_decr = true;
        let len = row.len();
        for i in 1..len {
            let diff = row[i] - row[i-1];
            if diff >= 1 && diff <= 3 {
                is_decr = false;
            } else if diff >= -3 && diff <= -1 {
                is_incr = false;
            } else {
                is_incr = false;
                is_decr = false;
            };
        };

        if is_incr || is_decr {
            ans += 1;
        };
    });

    fs::write("../../output/day-02-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
