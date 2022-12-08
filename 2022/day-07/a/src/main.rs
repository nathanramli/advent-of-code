use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Read,
    ops::AddAssign,
};

fn calculate_dir(
    path: String,
    dirs: &HashMap<String, HashSet<String>>,
    files_size: &HashMap<String, u64>,
    ans: &mut u64,
) -> u64 {
    let mut count = *files_size.get(&path).unwrap_or(&0);

    dirs.get(&path).unwrap().iter().for_each(|child_path| {
        count += calculate_dir(child_path.clone(), dirs, files_size, ans);
    });

    if count <= 100000 {
        *ans += count;
    }

    count
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("../../input/day-07.txt").expect("Error when reading input.");
    file.read_to_string(&mut input)
        .expect("Error when read as a string");

    let mut dirs = HashMap::new();
    dirs.insert("".to_string(), HashSet::new());

    let mut files_size = HashMap::new();
    files_size.insert("".to_string(), 0u64);

    let mut ans = 0;
    let mut curr_dir = "".to_string();
    input.split('\n').for_each(|row| {
        let mut splits = row.split_ascii_whitespace();
        let first_el = splits.next().unwrap();
        if first_el == "$" {
            let cmd = splits.next().unwrap();
            if cmd == "cd" {
                let path = splits.next().unwrap();
                if path == "/" {
                    curr_dir = "".to_string();
                } else if path == ".." {
                    curr_dir = curr_dir.rsplit_once("/").unwrap_or(("", "")).0.to_string();
                } else {
                    let parent_dir = curr_dir.clone();
                    if curr_dir != "" {
                        curr_dir.push_str("/");
                    }
                    curr_dir.push_str(path);

                    if dirs.get(&curr_dir).is_none() {
                        dirs.insert(curr_dir.clone(), HashSet::new());
                    }

                    if files_size.get(&curr_dir).is_none() {
                        files_size.insert(curr_dir.clone(), 0u64);
                    }

                    let hs = dirs.get_mut(&parent_dir).unwrap();
                    hs.insert(curr_dir.clone());
                }
            }
        } else if first_el == "dir" {
            let dir_name = splits.next().unwrap();
            let dir_path = {
                if curr_dir == "" {
                    dir_name.to_string()
                } else {
                    (curr_dir.clone() + "/" + dir_name).to_string()
                }
            };
            if dirs.get(&dir_path).is_none() {
                dirs.insert(dir_path.clone(), HashSet::new());
            }

            if files_size.get(&dir_path).is_none() {
                files_size.insert(dir_path.clone(), 0u64);
            }

            let hs = dirs.get_mut(&curr_dir).unwrap();
            hs.insert(dir_path.clone());
        } else {
            files_size
                .get_mut(&curr_dir)
                .unwrap()
                .add_assign(first_el.parse::<u64>().unwrap());
        }
    });

    calculate_dir("".to_string(), &dirs, &files_size, &mut ans);

    fs::write("../../output/day-07-part-1.txt", ans.to_string().as_bytes())
        .expect("error when writing the answer.");
}
