use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut cnt = vec![];
    let mut total = 0i64;

    input.trim().split('\n').for_each(|str| {
        let bin = str.trim().as_bytes();
        for (i, &el) in bin.iter().enumerate() {
            if cnt.len() <= i {
                cnt.push(0);
            }

            if el as char == '1' {
                cnt[i] += 1;
            }
        }
        total += 1;
    });

    let mut epsilon = 0i64;
    let mut gamma = 0;
    {
        for &i in &cnt {
            if total - i < i {
                gamma = gamma << 1 | 1;
                epsilon <<= 1;
            } else {
                gamma <<= 1;
                epsilon = epsilon << 1 | 1;
            }
        }
    };

    fs::write(
        "../../output/day-03-part-1.txt",
        (gamma * epsilon).to_string().as_bytes(),
    )
    .expect("error when writing the answer.");
}
