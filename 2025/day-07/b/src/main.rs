use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-07.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut map = vec![];
    let mut sum = vec![];

    let mut s_pos = Option::None;

    let mut i = 0;
    input.split('\n').for_each(|line: &str| {
        let chars = line.chars().collect::<Vec<char>>();

        let mut j = 0;
        chars.iter().for_each(|x| {
            if *x == 'S' {
                s_pos = Option::Some((i, j));
            }
            j += 1;
        });
        i += 1;

        map.push(chars);
    });

    let len_row = map.len();
    let len_col = map[0].len();

    for _ in 0..len_row {
        sum.push(vec![0; len_col]);
    }

    sum[s_pos.unwrap().0][s_pos.unwrap().1] = 1u64;

    for row in 1..len_row {
        for col in 0..len_col {
            if map[row][col] == '.' {
                if col + 1 < len_col && map[row][col + 1] == '^' {
                    sum[row][col] += sum[row - 1][col + 1];
                }

                if col >= 1 && map[row][col - 1] == '^' {
                    sum[row][col] += sum[row - 1][col - 1];
                }

                sum[row][col] += sum[row - 1][col];
            }
        }
    }

    for col in 0..len_col {
        ans += sum[len_row - 1][col];
    }

    fs::write("../../output/day-07-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
