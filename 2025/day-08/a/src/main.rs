use std::{
    fs::{self, File},
    io::Read,
};

fn dps(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, node: usize) -> i64 {
    let mut count = 1;
    visited[node] = true;
    for &neighbor in &adj[node] {
        if !visited[neighbor] {
            count += dps(adj, visited, neighbor);
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-08.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut ans = 0;
    let mut coords: Vec<(i64, i64, i64)> = vec![];
    let max_connected = 1000;

    input.split('\n').for_each(|line: &str| {
        let vec = line
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        coords.push((vec[0], vec[1], vec[2]));
    });

    let mut dist = vec![];

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

    let mut adj = vec![vec![]; coords.len()];
    let mut i = 0;
    while i < max_connected  {
        let u = dist[i].1.0;
        let v = dist[i].1.1;
        adj[u].push(v);
        adj[v].push(u);
        i += 1;
    }

    let mut sum = vec![];
    let mut visited = vec![false; coords.len()];
    
    for i in 0..coords.len() {
        if !visited[i] {
            sum.push(dps(&adj, &mut visited, i));
        }
    }

    sum.sort();
    sum.reverse();

    println!("{:?}", sum);

    ans = sum[0] * sum[1] * sum[2];

    fs::write("../../output/day-08-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
