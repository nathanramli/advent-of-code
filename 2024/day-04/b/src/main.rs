use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-04.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut map  = Vec::new();
    input.trim().split('\n').for_each(|row| {
        let chars = row.chars().collect::<Vec<char>>();
        map.push(chars);
    });

    let row_len = map.len();
    let col_len = map[0].len();

    for i in 0..row_len {
        for j in 0..col_len {
            let ch = map[i][j];
            if ch == 'A' {
                if i > 0 && j > 0 && i + 1 < row_len && j + 1 < col_len {
                    let mut sub = vec![map[i - 1][j - 1], map[i + 1][j + 1]];
                    let mut sub_2 = vec![map[i + 1][j - 1], map[i - 1][j + 1]];
                    sub.sort();
                    sub_2.sort();
                    if sub == vec!['M', 'S'] && sub_2 == vec!['M', 'S'] {
                        ans += 1;
                    }
                }
            }
        }
    }

    fs::write("../../output/day-04-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
