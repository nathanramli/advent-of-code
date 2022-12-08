use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-06.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut counts = vec![0; 26];
    let mut more_than_1 = 0;
    input.split('\n').for_each(|row| {
        let chars = row.chars().collect::<Vec<_>>();

        for i in 0..chars.len() {
            counts[(chars[i] as u32 - 97) as usize] += 1;
            if counts[(chars[i] as u32 - 97) as usize] == 2 {
                more_than_1 += 1;
            }

            if i >= 14 {
                counts[(chars[i - 14] as u32 - 97) as usize] -= 1;
                if counts[(chars[i - 14] as u32 - 97) as usize] == 1 {
                    more_than_1 -= 1;
                }
            }

            if i >= 13 && more_than_1 == 0 && ans == 0 {
                ans = i + 1;
            }
        }
    });

    fs::write("../../output/day-06-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
