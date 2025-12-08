use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
};

fn has_no_color(coloring: &Vec<i64>) -> bool {
    for &c in coloring {
        if c == -1 {
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-08.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut coords = vec![];

    input.split('\n').for_each(|line: &str| {
        let vec = line
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        coords.push((vec[0], vec[1], vec[2]));
    });

    let mut dist = vec![];
    let mut adj = vec![vec![]; coords.len()];

    for i in 0..coords.len() {
        let (x1, y1, z1) = coords[i];
        for j in (i + 1)..coords.len() {
            let (x2, y2, z2) = coords[j];
            let inner = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
            let dist_sq = (inner as f64).sqrt();
            dist.push((dist_sq, (i, j)));
        }
    }

    dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut coloring = vec![-1; coords.len()];
    let mut i = 0;
    let mut color = 1i64;
    let mut sets = HashSet::new();
    while i < dist.len() {
        let (_, (u, v)) = dist[i];
        i += 1;
        if coloring[u] == coloring[v] && coloring[u] != -1 {
            continue;
        }

        if coloring[u] != -1 && coloring[v] != -1 {
            let old_color = coloring[v].max(coloring[u]);
            let new_color = coloring[v].min(coloring[u]);
            for j in 0..coloring.len() {
                if coloring[j] == old_color {
                    coloring[j] = new_color;
                }
            }
            sets.remove(&old_color);
        } else if coloring[u] == -1 && coloring[v] == -1 {
            coloring[u] = color;
            coloring[v] = color;
            sets.insert(color);
            color += 1;
        } else if coloring[v] == -1 {
            coloring[v] = coloring[u];
        } else if coloring[u] == -1 {
            coloring[u] = coloring[v];
        }
        adj[u].push(v);

        let mut done = false;
        if !has_no_color(&coloring) && sets.len() == 1 {
            println!("{:?}", coords[u]);
            println!("{:?}", coords[v]);
            ans = coords[u].0 * coords[v].0;
            done = true;
            break;
        }
        if done {
            break;
        }
    }

    fs::write("../../output/day-08-part-2.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
