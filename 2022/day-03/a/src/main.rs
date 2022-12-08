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
    input.trim().split('\n').for_each(|row| {
        let input_row = row.trim().chars();
        let len = input_row.clone().count();
        let a = input_row.clone().take(len / 2).collect::<String>();
        let b = input_row.clone().skip(len / 2).collect::<String>();

        for i in a.chars() {
            if b.contains(i) {
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
    });

    fs::write("../../output/day-03-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
