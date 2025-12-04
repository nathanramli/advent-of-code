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

    let mut map: Vec<Vec<char>> = vec![];

    input.trim().split('\n').for_each(|line: &str| {
        let rows = line.chars().collect::<Vec<char>>();
        map.push(rows);
    });

    let row_len = map.len();
    let column_len = map[0].len();

    let adj = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        let mut found = false;
        for row in 0..row_len {
            for column in 0..column_len {
                let current_char = map[row][column];

                if current_char != '@' {
                    continue;
                }

                let mut cnt = 0;

                for adj in &adj {
                    let adj_row = row as i32 + adj.0;
                    let adj_column = column as i32 + adj.1;

                    if adj_row < 0
                        || adj_row >= row_len as i32
                        || adj_column < 0
                        || adj_column >= column_len as i32
                    {
                        continue;
                    }

                    if map[adj_row as usize][adj_column as usize] == '@' {
                        cnt += 1;
                    }
                }

                if cnt < 4 {
                    ans += 1;
                    found = true;
                    map[row][column] = '.';
                }
            }
        }

        if !found {
            break;
        }
    }


    fs::write("../../output/day-04-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
