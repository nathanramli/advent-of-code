use std::{
    fs::{self, File},
    io::Read, vec,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut map = Vec::new();
    input.trim().split('\n').for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        map.push(chars);
    });

    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];

    let len_row = map.len();
    let len_col = map[0].len();

    let mut current = 0;
    let mut adj = false;
    for i in 0..len_row {
        for j in 0..len_col {
            if map[i][j].is_numeric() {
                if current == 0  {
                    adj = false;
                }
                // found number
                current = current * 10 + map[i][j].to_digit(10).unwrap();

                if !adj {
                    for (dx, dy) in &dir {
                        let ni = i as isize + dx;
                        let nj = j as isize + dy;
                        if ni >= 0
                            && ni < len_row as isize
                            && nj >= 0
                            && nj < len_col as isize
                            && map[ni as usize][nj as usize] != '.'
                            && !map[ni as usize][nj as usize].is_numeric()
                        {
                            adj = true;
                        }
                    }
                }

                if j == len_col - 1 {
                    if adj {
                        ans += current;
                    }
                    current = 0;
                    adj = false;
                }
            } else {
                if adj {
                    ans += current;
                }
                adj = false;
                current = 0;
            }
        }
    }
    
    if adj {
        ans += current;
    }

    fs::write("../../output/day-03-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
