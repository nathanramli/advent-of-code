use std::{
    fs::{self, File},
    io::Read,
};

fn find_least_common<'a>(list: Vec<&'a [u8]>, at: &mut usize) -> Vec<&'a [u8]> {
    let mut _1s = 0;
    for &bin in &list {
        if *bin.get(*at).unwrap() as char == '1' {
            _1s += 1;
        }
    }

    let mut only = '0';
    if _1s < list.len() - _1s {
        only = '1';
    }

    let mut new_list: Vec<&'a [u8]> = vec![];
    for bin in list {
        if *bin.get(*at).unwrap() as char == only {
            new_list.push(bin);
        }
    }
    new_list
}

fn find_most_common<'a>(list: Vec<&'a [u8]>, at: &mut usize) -> Vec<&'a [u8]> {
    let mut _1s = 0;
    for &bin in &list {
        if *bin.get(*at).unwrap() as char == '1' {
            _1s += 1;
        }
    }

    let mut only = '1';
    if _1s < list.len() - _1s {
        only = '0';
    }

    let mut new_list: Vec<&'a [u8]> = vec![];
    for bin in list {
        if *bin.get(*at).unwrap() as char == only {
            new_list.push(bin);
        }
    }
    new_list
}

fn to_decimal(bin: &[u8]) -> i32 {
    let mut ans = 0;
    for &i in bin {
        ans <<= 1;
        if i as char == '1' {
            ans |= 1;
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-03.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let list = input
        .trim()
        .split('\n')
        .map(|str| str.trim().as_bytes())
        .collect::<Vec<_>>();

    let mut least_common_list = list.clone();
    let mut at = 0;
    let cO2_scrubber_rating;
    loop {
        least_common_list = find_least_common(least_common_list, &mut at);
        at += 1;
        if least_common_list.len() == 1 || at == least_common_list[0].len() {
            // immutable variable only can be assigned once
            // we haven't assigned any value when we creating this variable
            // thats why its not error
            cO2_scrubber_rating = to_decimal(least_common_list[0]);
            break;
        }
    }

    let mut most_common_list = list;
    at = 0;
    let oxygen_generator_rating;
    loop {
        most_common_list = find_most_common(most_common_list, &mut at);
        at += 1;
        if most_common_list.len() == 1 || at == most_common_list[0].len() {
            // immutable variable only can be assigned once
            // we haven't assigned any value when we creating this variable
            // thats why its not error
            oxygen_generator_rating = to_decimal(most_common_list[0]);
            break;
        }
    }

    fs::write(
        "../../output/day-03-part-2.txt",
        (cO2_scrubber_rating * oxygen_generator_rating)
            .to_string()
            .as_bytes(),
    )
    .expect("error when writing the answer.");
}
