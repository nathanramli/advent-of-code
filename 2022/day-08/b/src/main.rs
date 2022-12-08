use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-08.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut matrix = vec![];

    input.split('\n').for_each(|row| {
        matrix.push(vec![]);
        let len = matrix.len();
        row.chars().for_each(|x| {
            matrix[len - 1].push(x.to_string().parse::<i32>().unwrap());
        })
    });

    let count_distance_view =
        |mut row: usize, mut col: usize, move_row: i64, move_col: i64| -> u64 {
            let x = matrix[row][col];
            let mut cnt = 0;
            while row != 0 && row != matrix.len() - 1 && col != 0 && col != matrix[0].len() - 1 {
                row = (row as i64 + move_row) as usize;
                col = (col as i64 + move_col) as usize;

                cnt += 1;

                if matrix[row][col] >= x {
                    return cnt;
                }
            }
            cnt
        };

    let mut ans = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            ans = (count_distance_view(i, j, -1, 0)
                * count_distance_view(i, j, 1, 0)
                * count_distance_view(i, j, 0, -1)
                * count_distance_view(i, j, 0, 1))
            .max(ans);
        }
    }

    fs::write("../../output/day-08-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
