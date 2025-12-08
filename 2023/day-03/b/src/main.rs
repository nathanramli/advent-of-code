use std::{
    fs::{self, File},
    io::Read, vec,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0u64;

    let mut map = Vec::new();
    input.trim().split('\n').for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        map.push(chars);
    });

    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];

    let len_row = map.len();
    let len_col = map[0].len();

    let mut count_adjacent: Vec<Vec<u8>> = vec![vec![0; len_col]; len_row];
    let mut sum_adjacent: Vec<Vec<u64>> = vec![vec![0; len_col]; len_row];

    let mut on_sequence = false;
    let mut adj_vec = vec![];
    for i in 0..len_row {
        for j in 0..len_col {
            if map[i][j].is_numeric() {
                if !on_sequence {
                    on_sequence = true;
                    adj_vec.clear();
                }

                // found number
                for (dx, dy) in &dir {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;
                    if ni >= 0
                        && ni < len_row as isize
                        && nj >= 0
                        && nj < len_col as isize
                        && map[ni as usize][nj as usize] == '*'
                        && !adj_vec.contains(&(ni , nj))
                    {
                        adj_vec.push((ni, nj));
                        count_adjacent[ni as usize][nj as usize] += 1;
                    }
                }

                if j == len_col - 1 {
                    on_sequence = false;
                    adj_vec.clear();
                }
            } else {
                on_sequence = false;
                adj_vec.clear();
            }
        }
    }

    let mut current = 0;
    let mut adj = Option::None;
    for i in 0..len_row {
        for j in 0..len_col {
            if map[i][j].is_numeric() {
                if current == 0  {
                    adj = Option::None;
                }
                // found number
                current = current * 10 + map[i][j].to_digit(10).unwrap();

                if adj.is_none() {
                    for (dx, dy) in &dir {
                        let ni = i as isize + dx;
                        let nj = j as isize + dy;
                        if ni >= 0
                            && ni < len_row as isize
                            && nj >= 0
                            && nj < len_col as isize
                            && count_adjacent[ni as usize][nj as usize] == 2
                        {
                            adj = Option::Some((ni, nj));
                        }
                    }
                }

                if j == len_col - 1 {
                    if adj.is_some() {
                        let adj = adj.unwrap();
                        if sum_adjacent[adj.0 as usize][adj.1 as usize] == 0 {
                            sum_adjacent[adj.0 as usize][adj.1 as usize] += current as u64;
                        } else {
                            ans += (current as u64) * sum_adjacent[adj.0 as usize][adj.1 as usize];
                        }
                    }
                    current = 0;
                    adj = Option::None;
                }
            } else {
                if adj.is_some() {
                    let adj = adj.unwrap();
                    if sum_adjacent[adj.0 as usize][adj.1 as usize] == 0 {
                        sum_adjacent[adj.0 as usize][adj.1 as usize] += current as u64;
                    } else {
                        ans += (current as u64) * sum_adjacent[adj.0 as usize][adj.1 as usize];
                    }
                }
                adj = Option::None;
                current = 0;
            }
        }
    }
    
    if adj.is_some() {
        let adj = adj.unwrap();
        if sum_adjacent[adj.0 as usize][adj.1 as usize] == 0 {
            sum_adjacent[adj.0 as usize][adj.1 as usize] += current as u64;
        } else {
            ans += (current as u64) * sum_adjacent[adj.0 as usize][adj.1 as usize];
        }
    }

    fs::write("../../output/day-03-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
