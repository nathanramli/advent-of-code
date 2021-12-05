use std::{
    fs::{self, File},
    io::Read,
};

const N: i32 = 5;

fn find_and_mark(
    board: &mut Vec<Vec<i32>>,
    mark_x: &mut Vec<i32>,
    mark_y: &mut Vec<i32>,
    sum_board: &mut i32,
    number: i32,
) -> bool {
    let mut ans = false;
    'outside: for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] == number {
                mark_x[x] += 1;
                mark_y[y] += 1;

                *sum_board -= number;

                if mark_x[x] >= N || mark_y[y] >= N {
                    ans = true;
                    break 'outside;
                }
            }
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-04.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut iter_input = input.trim().split('\n');
    let numbers = iter_input
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut boards = vec![vec![vec![]]; 0];

    let mut number_of_board = 0;
    let mut number_of_row = 0;

    let mut board = vec![vec![0; N as usize]; N as usize];
    let mut sum = 0;
    let mut sum_boards = Vec::new();

    iter_input.skip(1).for_each(|input| {
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // Skipping blank space
        if row.len() != N as usize {
            return;
        }

        sum += &row.iter().sum();

        board[number_of_row] = row;
        number_of_row += 1;

        if number_of_row >= N as usize {
            number_of_row = 0;
            number_of_board += 1;
            boards.push(board.clone());
            sum_boards.push(sum);
            sum = 0;
        }
    });

    let mut mark_x = vec![vec![0; N as usize]; number_of_board];
    let mut mark_y = vec![vec![0; N as usize]; number_of_board];

    'outside: for number in numbers {
        for i in 0..number_of_board {
            let found = find_and_mark(
                &mut boards[i],
                &mut mark_x[i],
                &mut mark_y[i],
                &mut sum_boards[i],
                number,
            );
            if found {
                ans = number * sum_boards[i];
                break 'outside;
            }
        }
    }

    fs::write("../../output/day-04-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
