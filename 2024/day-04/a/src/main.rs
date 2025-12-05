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

    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let xmas = "MAS".chars().collect::<Vec<char>>();

    for i in 0..row_len {
        for j in 0..col_len {
            let ch = map[i][j];
            if ch == 'X' {
                for (dx, dy) in &dir {
                    let mut x = i as isize;
                    let mut y = j as isize;

                    let mut ok = true;
                    for k in 0..3 {
                        x += dx;
                        y += dy;

                        if x < 0 || x >= row_len as isize || y < 0 || y >= col_len as isize {
                            ok = false;
                            break;
                        }

                        if map[x as usize][y as usize] != xmas[k] {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        ans += 1;
                    }
                }
            }
        }
    }

    fs::write("../../output/day-04-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
