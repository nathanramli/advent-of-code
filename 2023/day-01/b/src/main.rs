use std::{
    fs::{self, File},
    io::Read,
};

fn find_first(s: &str, el: &str, asc: bool) -> i32 {
    let index = if asc {
        s.find(el).unwrap_or(usize::MAX)
    } else {
        s.rfind(el).unwrap_or(usize::MAX)
    };

    if index == usize::MAX {
        -1
    } else {
        index as i32
    }
}

fn main() {
    let dict = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut input = String::new();
    let mut file = File::open("../../input/day-01.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|x| {
        let mut first = (0, -1);
        let mut last = (0, -1);

        // words
        dict.iter().for_each(|&str| {
            let index = find_first(x, str.0, true);
            if index != -1 && (first.1 == -1 || first.1 > index) {
                first = (str.1, index);
            };

            let index = find_first(x, str.0, false);
            if index != -1 && (last.1 == -1 || last.1 < index) {
                last = (str.1, index);
            };
        });

        // digits
        for el in '1'..='9' {
            let index = find_first(x, &el.to_string(), true);
            if index != -1 && (first.1 == -1 || first.1 > index) {
                first = (el.to_string().parse::<i32>().unwrap(), index);
            };

            let index = find_first(x, &el.to_string(), false);
            if index != -1 && (last.1 == -1 || last.1 < index) {
                last = (el.to_string().parse::<i32>().unwrap(), index);
            };
        }

        ans += first.0 * 10 + last.0;
    });

    fs::write("../../output/day-01-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
