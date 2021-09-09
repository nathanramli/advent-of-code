use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Debug)]
struct Waypoint {
    north: i32,
    east: i32,
    south: i32,
    west: i32,
}

#[derive(Debug)]
struct Ship {
    north: i32,
    east: i32,
    south: i32,
    west: i32,
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-12.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut waypoint = Waypoint {
        north: 1,
        east: 10,
        south: 0,
        west: 0,
    };

    let mut ship = Ship {
        north: 0,
        east: 0,
        south: 0,
        west: 0,
    };

    input.trim().split('\n').for_each(|x| {
        let x = x.trim(); // Trim it again

        let command = x.get(0..1).unwrap();
        let value = x.get(1..(x.len())).unwrap().parse::<i32>();
        if value.is_err() {
            panic!("Panic on input {:?}", &x);
        }
        let value = value.unwrap();

        match command {
            "N" => waypoint.north += value,
            "E" => waypoint.east += value,
            "S" => waypoint.south += value,
            "W" => waypoint.west += value,
            "L" => {
                let shift = value / 90;
                for _ in 0..shift {
                    let temp = waypoint.north;
                    waypoint.north = waypoint.east;
                    waypoint.east = waypoint.south;
                    waypoint.south = waypoint.west;
                    waypoint.west = temp;
                }
            }
            "R" => {
                let shift = value / 90;
                for _ in 0..shift {
                    let temp = waypoint.north;
                    waypoint.north = waypoint.west;
                    waypoint.west = waypoint.south;
                    waypoint.south = waypoint.east;
                    waypoint.east = temp;
                }
            }
            "F" => {
                ship.north += waypoint.north * value;
                ship.west += waypoint.west * value;
                ship.south += waypoint.south * value;
                ship.east += waypoint.east * value;
            }
            _ => (), // we both can use {} or ()
        };
    });

    let ans = i32::abs(ship.west - ship.east) + i32::abs(ship.north - ship.south);

    fs::write("../../output/day-12-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
