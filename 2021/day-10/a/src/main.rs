use std::{
    collections::VecDeque,
    fs::{self, File},
    io::Read,
};

fn get_score(x: char) -> i64 {
    match x {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-10.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    input.trim().split('\n').for_each(|buff| {
        let lines = buff.trim();
        let mut q = VecDeque::new();

        'iterator: for i in lines.as_bytes() {
            match *i as char {
                ')' => {
                    let open = q.pop_back().unwrap();
                    if open != '(' {
                        ans += get_score(*i as char);
                        break 'iterator;
                    }
                }
                ']' => {
                    let open = q.pop_back().unwrap();
                    if open != '[' {
                        ans += get_score(*i as char);
                        break 'iterator;
                    }
                }
                '}' => {
                    let open = q.pop_back().unwrap();
                    if open != '{' {
                        ans += get_score(*i as char);
                        break 'iterator;
                    }
                }
                '>' => {
                    let open = q.pop_back().unwrap();
                    if open != '<' {
                        ans += get_score(*i as char);
                        break 'iterator;
                    }
                }
                i => q.push_back(i),
            }
        }
    });

    fs::write("../../output/day-10-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
