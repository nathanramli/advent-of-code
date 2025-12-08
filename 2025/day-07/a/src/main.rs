use std::{
    fs::{self, File},
    io::Read,
};

fn flooding(maps: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize, len_row: usize, len_col: usize) {
    if x >= len_row || y >= len_col || visited[x][y] {
        return;
    }
    visited[x][y] = true;
    if maps[x][y] != '^' {
        maps[x][y] = '|';

        flooding(maps, visited, x + 1, y, len_row, len_col);
    } else if maps[x][y] == '^' {
        flooding(maps, visited, x, y + 1, len_row, len_col);        
        flooding(maps, visited, x, y - 1, len_row, len_col);        
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-07.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    let mut map = vec![];
    let mut visited = vec![];

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
        visited.push(vec![false; len_col]);
    }
    
    flooding(&mut map, &mut visited, s_pos.unwrap().0, s_pos.unwrap().1, len_row, len_col);

    for row in 0..len_row {
        for col in 0..len_col {
            if row > 0 && map[row][col] == '^' && map[row-1][col] == '|' {
                ans += 1;
            }
        }
    }

    fs::write("../../output/day-07-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
