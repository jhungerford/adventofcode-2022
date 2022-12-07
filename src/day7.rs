use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let root = parse_output("input/day7.txt");

    println!("Day 7");
    println!("Part 1: {}", total_small(&root));
    println!("Part 2: {}", delete_dir(&root, 70_000_000, 30_000_000));
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
            get_dir(&mut root, &path).size += file_size;
        }
    }

    // dir size currently only counts the files that are immediately in a directory.
    // Both parts want the recursive size (files + child dir sizes) - update size.

    // add_child_size adds the recursive size of child dirs to the given directory's size,
    // and returns the directory's recursive size.
    fn add_child_size(dir: &mut Dir) -> u64 {
        dir.size = dir.children.iter_mut().fold(dir.size, |size, (_, child_dir)| {
            size + add_child_size(child_dir)
        });

        dir.size
    }

    add_child_size(&mut root);

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
    size: u64,
    children: HashMap<String, Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        return Dir {
            _name: name,
            size: 0,
            children: HashMap::new(),
        }
    }
}

/// total_small returns the total size of directories less than 100_000 bytes.
/// Directory size is recursive, so file sizes can be counted more than once.
fn total_small(dir: &Dir) -> u64 {
    let mut total = 0;

    if dir.size < 100_000 {
        total += dir.size;
    }

    for (_, child) in &dir.children {
        total += total_small(child);
    }

    total
}

/// delete_dir returns the size of the smallest directory to delete that will free enough space.
fn delete_dir(root: &Dir, total: u64, need: u64) -> u64 {
    // find the size of the smallest directory that frees up enough space to reach need.

    fn smallest_free_dir(dir: &Dir, to_free: u64, smallest: u64) -> u64 {
        let mut smallest = smallest;

        if dir.size > to_free && dir.size < smallest {
            smallest = dir.size;
        }

        for (_, child_dir) in &dir.children {
            smallest = smallest_free_dir(child_dir, to_free, smallest);
        }

        smallest
    }

    let to_free = need - (total - root.size);
    smallest_free_dir(root, to_free, u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_small() {
        let root = parse_output("input/day7_sample.txt");

        assert_eq!(95437, total_small(&root));
    }

    #[test]
    fn test_delete_dir() {
        let root = parse_output("input/day7_sample.txt");

        assert_eq!(24933642, delete_dir(&root, 70_000_000, 30_000_000));
    }
}
