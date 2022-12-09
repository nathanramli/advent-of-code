use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-09.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut marks = HashSet::new();
    marks.insert((0, 0));

    let mut knots = vec![(0, 0); 10];

    let is_around = |h_pos: &(i32, i32), t_pos: &(i32, i32)| -> bool {
        t_pos.0.abs_diff(h_pos.0) <= 1 && t_pos.1.abs_diff(h_pos.1) <= 1
    };

    let move_t = |h_pos: &(i32, i32), t_pos: &mut (i32, i32)| {
        if t_pos.0 != h_pos.0 && t_pos.1 != h_pos.1 {
            for i in vec![(1, 1), (1, -1), (-1, 1), (-1, -1)].iter() {
                t_pos.0 += i.0;
                t_pos.1 += i.1;

                if is_around(&h_pos, &t_pos) {
                    break;
                }

                t_pos.0 -= i.0;
                t_pos.1 -= i.1;
            }
        } else {
            for i in vec![(0, 1), (0, -1), (-1, 0), (1, 0)].iter() {
                t_pos.0 += i.0;
                t_pos.1 += i.1;

                if is_around(&h_pos, &t_pos) {
                    break;
                }

                t_pos.0 -= i.0;
                t_pos.1 -= i.1;
            }
        }
    };

    input.split('\n').for_each(|row| {
        let mut input_col = row.split_ascii_whitespace();
        let direction = input_col.next().unwrap();
        let dirs = match direction {
            "R" => (0, 1),
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            _ => (0, 0),
        };

        let step = input_col.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..step {
            knots[0].0 += dirs.0;
            knots[0].1 += dirs.1;

            for i in 1..10 {
                if !is_around(&knots[i - 1], &knots[i]) {
                    let head = knots[i - 1].clone();
                    move_t(&head, &mut knots[i]);

                    if i == 9 {
                        marks.insert(knots[i].clone());
                    }
                }
            }
        }
    });

    fs::write(
        "../../output/day-09-part-2.txt",
        marks.len().to_string().as_bytes(),
    )
    .expect("error when writing the answer.");
}
