use std::{
    collections::VecDeque,
    fs::{self, File},
    io::Read,
};

fn get_score(bracket: char) -> i64 {
    match bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn get_close_bracket(open_bracket: char) -> char {
    match open_bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => '-',
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-10.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = Vec::new();
    input.trim().split('\n').for_each(|buff| {
        let lines = buff.trim();
        let mut q = VecDeque::new();

        let mut breaked = false;
        'iterator: for i in lines.as_bytes() {
            match *i as char {
                ')' => {
                    let &open = q.back().unwrap();
                    if open != '(' {
                        breaked = true;
                        break 'iterator;
                    }
                    q.pop_back();
                }
                ']' => {
                    let &open = q.back().unwrap();
                    if open != '[' {
                        breaked = true;
                        break 'iterator;
                    }
                    q.pop_back();
                }
                '}' => {
                    let &open = q.back().unwrap();
                    if open != '{' {
                        breaked = true;
                        break 'iterator;
                    }
                    q.pop_back();
                }
                '>' => {
                    let &open = q.back().unwrap();
                    if open != '<' {
                        breaked = true;
                        break 'iterator;
                    }
                    q.pop_back();
                }
                i => q.push_back(i),
            }
        }

        if !breaked {
            let mut sum = 0;
            for &i in q.iter().rev() {
                sum *= 5;
                sum += get_score(get_close_bracket(i));
            }
            ans.push(sum);
        }
    });

    ans.sort();
    let ans = ans[ans.len() / 2];

    fs::write("../../output/day-10-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
