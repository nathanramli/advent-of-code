use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
};

//      1
//  2		3
// 	    4
//  5		7
// 	    6

// to find 1 = find a character in 3s that not exist in 2s
// to find 3 = find a character in 2s that not exist in a 6s
// to find 7 = its the remaining character from 2s (to find 3)
// to find 5 = find a missing character in a 6s, that the missing character not exists in 4s
// to find 4 = missing character in 6s, the 6s that not used in "to find 3" and "to find 5"
// to find 6 = a character in the last 2 missing character, that not exist in 4s
// to find 2 = the remaining character

fn character_not_exists_in(from: &String, not_in: &String) -> Vec<char> {
    let mut ans = Vec::new();
    for ch in from.chars() {
        if !not_in.contains(ch) {
            ans.push(ch);
        }
    }
    ans
}

fn recognize(pattern: Vec<String>) -> HashMap<char, i32> {
    let mut solved_pattern = HashMap::new();

    // find 1
    'find_1: for i in &pattern {
        if i.len() == 3 {
            for j in &pattern {
                if j.len() == 2 {
                    solved_pattern.insert(character_not_exists_in(i, j)[0], 1);
                    break 'find_1;
                }
            }
        }
    }

    let mut used_6s = vec![];

    // find 3
    'find_3: for i in &pattern {
        if i.len() == 2 {
            for j in &pattern {
                if j.len() == 6 {
                    let not_exists = character_not_exists_in(i, j);
                    if not_exists.is_empty() {
                        continue;
                    }

                    used_6s.push(j.to_string());

                    let mut chars = i.chars();
                    let first = chars.next().unwrap();
                    let second = chars.next().unwrap();

                    if first == not_exists[0] {
                        solved_pattern.insert(second, 7);
                    } else {
                        solved_pattern.insert(first, 7);
                    }

                    solved_pattern.insert(not_exists[0], 3);
                    break 'find_3;
                }
            }
        }
    }

    let mut _4s = String::new();
    // find 5
    'find_5: for i in &pattern {
        if i.len() == 4 {
            _4s = i.clone();
            for j in &pattern {
                if j.len() == 6 {
                    let not_exists = character_not_exists_in(&"abcdefg".to_string(), j);
                    if !i.contains(not_exists[0]) {
                        used_6s.push(j.to_string());

                        solved_pattern.insert(not_exists[0], 5);
                        break 'find_5;
                    }
                }
            }
        }
    }

    // find 6
    'find_6: for i in &pattern {
        if i.len() == 6 && !used_6s.contains(i) {
            for j in "abcdefg".chars() {
                if !i.contains(j) {
                    solved_pattern.insert(j, 4);
                    break 'find_6;
                }
            }
        }
    }

    for i in "abcdefg".chars() {
        if solved_pattern.get(&i).is_none() {
            if !_4s.contains(i) {
                solved_pattern.insert(i, 6);
            } else {
                solved_pattern.insert(i, 2);
            }
        }
    }

    solved_pattern
}

fn what_number_is_this(
    number_in_pattern: &str,
    solved_pattern: &HashMap<char, i32>,
    real_pattern: &HashMap<&str, i32>,
) -> i32 {
    let mut pattern = Vec::new();
    for i in number_in_pattern.chars() {
        pattern.push(
            solved_pattern
                .get(&i)
                .unwrap()
                .to_string()
                .chars()
                .next()
                .unwrap(),
        );
    }
    pattern.sort();
    let pattern_sorted = pattern.iter().collect::<String>();
    *real_pattern.get(pattern_sorted.as_str()).unwrap()
}

fn main() {
    let mut real_pattern = HashMap::new();
    // Insert the real pattern
    real_pattern.insert("123567", 0);
    real_pattern.insert("37", 1);
    real_pattern.insert("13456", 2);
    real_pattern.insert("13467", 3);
    real_pattern.insert("2347", 4);
    real_pattern.insert("12467", 5);
    real_pattern.insert("124567", 6);
    real_pattern.insert("137", 7);
    real_pattern.insert("1234567", 8);
    real_pattern.insert("123467", 9);

    let mut input = String::new();
    let mut file = File::open("../../input/day-08.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    input.trim().split('\n').for_each(|buff| {
        let mut splits = buff.split('|');
        let input = splits
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let solved_pattern = recognize(input);

        let output = splits.next().unwrap().trim().split_ascii_whitespace();
        for (index, pattern) in output.enumerate() {
            ans += what_number_is_this(pattern, &solved_pattern, &real_pattern)
                * 10i32.pow(3 - index as u32);
        }
    });

    fs::write("../../output/day-08-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
