use std::{fs::{self, File}, io::Read};

fn simulate(map: &mut Vec<Vec<u8>>, height: usize, width: usize, going_sit: bool) -> bool {
    let mut cmd: Vec<(usize, usize, u8)> = Vec::new();

    let mut changed = false;
    for i in 0..height {
        for j in 0..width {
            if going_sit && map[i][j] != 'L' as u8 {
                continue;
            } else if !going_sit && map[i][j] != '#' as u8 {
                continue;
            }
            let mut cnt = 0;

            for x in -1..=1_i32 {
                for y in -1..=1_i32 {
                    if y == 0 && x == 0 {
                        continue;
                    }

                    let row = i as i32 + x;
                    let col = j as i32 + y;
                    if row >= 0 && row < height as i32 && col >= 0 && col < width as i32 {
                        if going_sit {
                            if cnt == 0 && map[row as usize][col as usize] == '#' as u8 {
                                cnt = 1;
                            }
                        } else {
                            if map[row as usize][col as usize] == '#' as u8 {
                                cnt += 1;
                                if cnt == 4 {
                                    changed = true;
                                    cmd.push((i, j, 'L' as u8));
                                }
                            }
                        }
                    }
                }
            }
            if going_sit && cnt == 0 {
                changed = true;
                cmd.push((i, j, '#' as u8));
            }
        }
    }

    while let Some(c) = cmd.pop() {
        map[c.0][c.1] = c.2;
    }

    changed
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-11.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut height = 0;
    let mut width = 0;

    let mut map: Vec<Vec<u8>> = Vec::new();

    input.trim().split('\n').for_each(|x| {
        let input = x.trim().to_string();

        if width == 0 {
            width = input.chars().count();
        }
        height += 1;

        map.push(input.as_bytes().to_vec());
    });

    let mut going_sit = true;
    while simulate(&mut map, height, width, going_sit) {
        going_sit = !going_sit;
    }

    let mut ans = 0;
    for i in 0..height {
        for j in 0..width {
            if map[i][j] == '#' as u8 {
                ans += 1;
            }
        }
    }

    fs::write("../../output/day-11-part-1.txt", ans.to_string().as_bytes()).expect("error when writing the answer.");
}
