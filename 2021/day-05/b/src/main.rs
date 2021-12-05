use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Debug)]
struct Coordinates {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-05.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut lines: Vec<Coordinates> = Vec::new();

    let mut max_y = 0;
    let mut max_x = 0;
    input.trim().split('\n').for_each(|str| {
        let mut line = str.trim().split(" -> ");
        let (x1, y1) = line.next().unwrap().split_once(',').unwrap();
        let (x2, y2) = line.next().unwrap().split_once(',').unwrap();

        let x1 = x1.parse::<i32>().unwrap();
        let y1 = y1.parse::<i32>().unwrap();
        let x2 = x2.parse::<i32>().unwrap();
        let y2 = y2.parse::<i32>().unwrap();

        let coordinates = Coordinates{
            x1,
            y1,
            x2,
            y2,
        };

        max_x = std::cmp::max(std::cmp::max(x1, x2), max_x);
        max_y = std::cmp::max(std::cmp::max(y1, y2), max_y);

        lines.push(coordinates);
    });

    let mut vis = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];
    lines
    .iter()
    .for_each(|coords| {
        if coords.x1 == coords.x2 {
            let from = std::cmp::min(coords.y1, coords.y2) as usize;
            let to = std::cmp::max(coords.y1, coords.y2) as usize;

            vis
            .iter_mut()
            .take(to + 1)
            .skip(from)
            .for_each(|val|
            {

                if val[coords.x1 as usize] == 1 {
                    ans += 1;
                } 
                val[coords.x1 as usize] += 1;
            });

        } else if coords.y1 == coords.y2 {

            let from = std::cmp::min(coords.x1, coords.x2) as usize;
            let to = std::cmp::max(coords.x1, coords.x2) as usize;

            vis[coords.y1 as usize]
            .iter_mut()
            .take(to + 1)
            .skip(from)
            .for_each(|val|
            {
                if *val == 1 {
                    ans += 1;
                } 
                *val += 1;
            });
        } else {
            let step_x = {
                if coords.x1 < coords.x2 {
                    1
                } else {
                    -1
                }
            };

            let step_y = {
                if coords.y1 < coords.y2 {
                    1
                } else {
                    -1
                }
            };

            let mut x = coords.x1;
            let mut y = coords.y1;
            for _ in 0..=(coords.x1 - coords.x2).abs() {
                let val = &mut vis[y as usize][x as usize];
                if *val == 1 {
                    ans += 1;
                }
                *val += 1;
                x += step_x;
                y += step_y;
            }
        }
    });

    fs::write(
        "../../output/day-05-part-2.txt",
        ans.to_string().as_bytes(),
    )
    .expect("error when writing the answer.");
}
