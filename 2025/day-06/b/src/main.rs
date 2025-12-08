use std::{
    fs::{self, File},
    io::Read,
};

// 1. make it as map
// 2. loop from top to bottom, left to right
// 3. when found a new number, the previous number multiply by 10 and add the new number
// 4. if found a symbol * or +, at the end of row, the next number will do arithmentic with the current number
// 5. if found a new symbol, the sum will be reset from 0, and repeat step 4
fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-06.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut map = Vec::new();

    input.split('\n').for_each(|line: &str| {
        map.push(line.chars().collect::<Vec<char>>());
    });

    let len_row = map.len();
    let len_col = map[0].len();

    let mut symbol = '-';
    let mut current_sum = 0;
    for col in 0..len_col {
        let mut found = false;
        let mut full_number = 0;
        for row in 0..len_row {
            let ch = map[row][col];
            if ch.is_numeric() {
                found = true;
                let num = ch.to_digit(10).unwrap() as u64;
                full_number = full_number * 10 + num;
            }

            if row == len_row - 1 && found {
                if ch == '*' {
                    symbol = '*';
                    ans += current_sum;
                    current_sum = 0;
                } else if ch == '+' {
                    symbol = '+';
                    ans += current_sum;
                    current_sum = 0;
                }

                if symbol == '+' {
                    current_sum += full_number;
                } else {
                    if current_sum == 0 {
                        current_sum = full_number;
                    } else {
                        current_sum *= full_number;
                    }
                }
            }
        }
    }
    ans += current_sum;

    fs::write("../../output/day-06-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
