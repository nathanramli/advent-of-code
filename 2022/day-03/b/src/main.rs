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
    let mut first = "";
    let mut second = "";
    let mut third = "";
    let mut i = 0;
    input.trim().split('\n').for_each(|row| {
        if i % 3 == 0 {
            first = row;
        } else if i % 3 == 1 {
            second = row;
        } else {
            third = row;

            for i in third.chars() {
                if second.contains(i) && first.contains(i) {
                    ans += {
                        if i.is_lowercase() {
                            (i as u32) as i32 - 96
                        } else {
                            (i as u32) as i32 - 64 + 26
                        }
                    };
                    break;
                }
            }
        }
        i += 1;
    });

    fs::write("../../output/day-03-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
