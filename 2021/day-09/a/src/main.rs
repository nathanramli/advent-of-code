use std::{
    fs::{self, File},
    io::Read,
};
fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-09.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut map = Vec::new();

    input.trim().split('\n').for_each(|buff| {
        let mut row = vec![];
        buff.trim().as_bytes().iter().for_each(|f| row.push(f - 48));
        map.push(row);
    });

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let x = map[i][j];
            if i > 0 && map[i - 1][j] <= x {
                continue;
            }
            if j > 0 && map[i][j - 1] <= x {
                continue;
            }
            if j + 1 < map[i].len() && map[i][j + 1] <= x {
                continue;
            }
            if i + 1 < map.len() && map[i + 1][j] <= x {
                continue;
            }
            ans += x as i64 + 1;
        }
    }

    fs::write("../../output/day-09-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
