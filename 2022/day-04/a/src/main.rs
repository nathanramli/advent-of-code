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
    input.trim().split('\n').for_each(|row| {
        let mut input_row = row.trim().split(",");
        let mut left_range = input_row.next().unwrap().split("-");
        let mut right_range = input_row.next().unwrap().split("-");

        let ll = left_range.next().unwrap().parse::<i32>().unwrap();
        let lr = left_range.next().unwrap().parse::<i32>().unwrap();

        let rl = right_range.next().unwrap().parse::<i32>().unwrap();
        let rr = right_range.next().unwrap().parse::<i32>().unwrap();

        if (ll <= rl && lr >= rr) || (rl <= ll && rr >= lr) {
            ans += 1;
        }
    });

    fs::write("../../output/day-04-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
