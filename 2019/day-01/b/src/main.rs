use std::{fs::{self, File}, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-01.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let calculate = |mass: i32| -> i32 {
        mass / 3  - 2
    };

    let mut sum = 0;
    input.trim().split('\n').for_each(|str| {
        let input = str.trim().to_string();
        
        let mut mass = input.parse::<i32>().unwrap();
        while mass > 0 {
            let temp = std::cmp::max(calculate(mass), 0);
            mass = temp;
            sum += temp;
        }
    });
    
    fs::write("../../output/day-01-part-2.txt", sum.to_string().as_bytes()).expect("error when writing the answer.");
}
