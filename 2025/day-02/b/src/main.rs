use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-02.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;

    input.trim().split('\n').for_each(|line: &str| {
        let ranges = line
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        
        let range = ranges.iter().map(|f| {
            let mut iter = f.split('-');
            let begin = iter.next().unwrap().parse::<u64>().unwrap();
            let end = iter.next().unwrap().parse::<u64>().unwrap();

            (begin, end)
        }).collect::<Vec<(u64, u64)>>();

        range.iter().for_each(|v| {
            let first = v.0;
            let end = v.1;

            for number in first..=end {
                let str = number.to_string();

                for length in 1..=str.len() {
                    if str.len() % length != 0{
                        continue;
                    }

                    if str.len() / length < 2 {
                        break;
                    }

                    let chunks_size = str.len() / length;
                    let chars = str.chars().into_iter().collect::<Vec<char>>();
                    let mut chunks = chars.chunks(length);
                    let first_chunk = chunks.next().unwrap();
                    let mut ok = true;
                    for _ in 1..chunks_size {
                        let chunk = chunks.next().unwrap();
                        if chunk != first_chunk {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        ans += number;
                        break;
                    }
                }
            }
        });
    });

    fs::write("../../output/day-02-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
