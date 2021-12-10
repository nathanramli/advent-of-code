use std::{
    fs::{self, File},
    io::Read,
};

fn sn(n: i32) -> i32 {
    (n as f32 / 2. * (1 + n) as f32) as i32
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-07.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = i32::MAX;
    let mut numbers = vec![];
    let mut mx = 0;
    let mut mn = i32::MAX;
    input.trim().split(',').for_each(|buff| {
        let x = buff.parse::<i32>().unwrap();
        mx = std::cmp::max(x, mx);
        mn = std::cmp::min(x, mn);
        numbers.push(x);
    });

    for i in mn..=mx {
        let mut sm = 0;
        for j in &numbers {
            sm += sn((i - j).abs());
        }
        ans = std::cmp::min(sm, ans);
    }

    fs::write("../../output/day-07-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
