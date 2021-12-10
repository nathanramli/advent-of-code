use std::{
    fs::{self, File},
    io::Read,
};

fn find_basin(x: usize, y: usize, map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>) -> i64 {
    let mut ans = 1;
    visited[x][y] = true;

    if x > 0 && map[x - 1][y] != 9 && !visited[x - 1][y] {
        ans += find_basin(x - 1, y, map, visited);
    }
    if y > 0 && map[x][y - 1] != 9 && !visited[x][y - 1] {
        ans += find_basin(x, y - 1, map, visited);
    }
    if y + 1 < map[x].len() && map[x][y + 1] != 9 && !visited[x][y + 1] {
        ans += find_basin(x, y + 1, map, visited);
    }
    if x + 1 < map.len() && map[x + 1][y] != 9 && !visited[x + 1][y] {
        ans += find_basin(x + 1, y, map, visited);
    }
    ans
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-09.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut map = Vec::new();

    input.trim().split('\n').for_each(|buff| {
        let mut row = vec![];
        buff.trim().as_bytes().iter().for_each(|f| row.push(f - 48));
        map.push(row);
    });

    let mut basins = Vec::new();

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
            basins.push(find_basin(
                i,
                j,
                &map,
                &mut vec![vec![false; map[i].len()]; map.len()],
            ));
        }
    }

    
    let mut ans = 1;
    basins.sort();
    for i in basins.iter().rev().take(3) {
        ans *= i;
    }

    fs::write("../../output/day-09-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
