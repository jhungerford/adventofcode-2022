use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let root = parse_output("input/day7.txt");

    println!("Day 7");
    println!("Part 1: {}", total_small(&root))
}

fn parse_output(filename: &str) -> Dir {
    let mut path: VecDeque<String> = VecDeque::new();
    let mut root = Dir::new("/".to_string());

    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    for line in f.lines().flatten() {
        if line == "$ cd .." {
            // $ cd .. - go up one level
            path.pop_back();

        } else if line.starts_with("$ cd") {
            // $ cd a - go into a directory
            let dir_name = line.replace("$ cd ", "");

            if dir_name != "/" {
                path.push_back(dir_name);
            }

        } else if line.starts_with("$ ls") {
            // $ ls - list files (nothing to do in the parser - next lines will be dirs and files)

        } else if line.starts_with("dir") {
            // dir d - record a directory
            let dir_name = line.replace("dir ", "");
            get_dir(&mut root, &path).children.insert(dir_name.clone(), Dir::new(dir_name.clone()));

        } else if !line.is_empty() {
            // 8033020 d.log - add to the total size of this dir
            let file_size = line.split_whitespace().next().unwrap().parse::<u64>().unwrap();
            get_dir(&mut root, &path).files_size += file_size;
        }
    }

    root
}

fn get_dir<'a>(root: &'a mut Dir, path: &VecDeque<String>) -> &'a mut Dir {
    let mut dir = root;

    for dir_name in path {
        dir = dir.children.get_mut(dir_name).unwrap();
    }

    dir
}

#[derive(Debug)]
struct Dir {
    _name: String,
    files_size: u64,
    children: HashMap<String, Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        return Dir {
            _name: name,
            files_size: 0,
            children: HashMap::new(),
        }
    }
}

fn total_small(root: &Dir) -> u64 {
    // total_small_sub returns the total size of the directory and it's children (recursive)
    // as well as the total size of small subdirectories.
    fn total_small_sub(dir: &Dir) -> (u64, u64) {
        let mut dir_size = dir.files_size;
        let mut total = 0;

        for (_, dir) in &dir.children {
            let (child_size, sub_total) = total_small_sub(dir);
            dir_size += child_size;
            total += sub_total;
        }

        if dir_size < 100_000 {
            total += dir_size;
        }

        (dir_size, total)
    }

    total_small_sub(root).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_small() {
        let root = parse_output("input/day7_sample.txt");

        assert_eq!(95437, total_small(&root))
    }
}