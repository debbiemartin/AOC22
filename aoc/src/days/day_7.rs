use crate::problem::Problem;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;

pub struct DaySeven {}

#[allow(dead_code)]
struct File {
    name: String,
    size: u32,
}

#[derive(Default)]
struct Directory {
    subdirs: Vec<PathBuf>,
    files: Vec<File>,
}

fn get_size_dir(
    dirs: &HashMap<PathBuf, Directory>, 
    dirpath: &PathBuf,
    sizes: &mut HashMap<PathBuf, u32>
) -> u32{
    if let Some(x) = sizes.get(dirpath) {
        return *x
    }
    
    let dir = dirs.get(dirpath).unwrap();
    let mut size = 0;
    for subdir in &dir.subdirs {
        size += get_size_dir(dirs, subdir, sizes);
    }
    
    for file in &dir.files {
        size += file.size;
    }
    
    sizes.insert(dirpath.clone(), size);
    
    size
}

fn get_sizes(dirs: &HashMap<PathBuf, Directory>) -> HashMap<PathBuf, u32> {
    let mut sizes = HashMap::new();
    for (dir_path, _) in dirs {
        get_size_dir(dirs, dir_path, &mut sizes);
    }
    sizes
}
        
fn change_dir(dirs: &mut HashMap<PathBuf, Directory>, current_path: &PathBuf, relative_path: &str) -> PathBuf {
    let path: PathBuf = match relative_path {
        "/" => Path::new("/").to_path_buf(),
        ".." => current_path.parent().unwrap().to_path_buf(),
        _ => current_path.join(relative_path),
    };
    path.to_str().unwrap();

    dirs.entry(path.clone()).or_default();
    path
}

fn add_file(dirs: &mut HashMap<PathBuf, Directory>, dir_path: &PathBuf, file: &str)  {
    let dir: &mut Directory = dirs.get_mut(dir_path).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<size>[0-9]+)\s(?P<name>[a-z.]+)\s*$").unwrap();
    }
    
    let cap = RE.captures(file.trim()).unwrap();
    let size: u32 = cap.name("size").unwrap().as_str().parse::<u32>().unwrap();
    let name: String = String::from(cap.name("name").unwrap().as_str());
        
    dir.files.push(File { name: name, size: size })
}

fn add_subdir(dirs: &mut HashMap<PathBuf, Directory>, dir_path: &PathBuf, subdir: &str) {
    let subdir_filepath = dir_path.join(subdir);
    let dir: &mut Directory = dirs.get_mut(dir_path).unwrap();
    dir.subdirs.push(subdir_filepath.clone());
    dirs.entry(subdir_filepath).or_default();
}

fn parse(input: &str) -> HashMap<PathBuf, u32> {
    let mut dirs: HashMap<PathBuf, Directory> = HashMap::new();
    let mut current_filepath = Path::new("/").to_path_buf();
    for output in input.lines() {
        if output.starts_with("$ cd") {
            current_filepath = change_dir(&mut dirs, &current_filepath, output[4..].trim());
        } else if output.starts_with("$ ls") {
            continue
        } else {
            if output.starts_with("dir") {
                add_subdir(&mut dirs, &current_filepath, &output[4..]);
            } else {
                add_file(&mut dirs, &current_filepath, output);
            }
        }
    }
    
    get_sizes(&dirs)
}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let sizes = parse(input);
        let mut total = 0;
        for size in sizes.values() {
            if *size < 100000 {
                total += size;
            }
        }
        format!("Sum of sizes below 100000: {total}")
    }

    fn part_two(&self, input: &str) -> String {
        let sizes = parse(input);
        let total: u32 = *sizes.get(&Path::new("/").to_path_buf()).unwrap();
        let space_to_free = total - 40000000;
        let mut size_vec: Vec<&u32> = sizes.values().collect();
        size_vec.sort_by(|a, b| (*a).cmp(*b));
    
        for value in size_vec {
            if *value > space_to_free {
                return format!("Size to be deleted: {value}");
            }
        }
        panic!("Didn't find any directories large enough")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_p1() {
        let input="$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(DaySeven{}.part_one(&input), "Sum of sizes below 100000: 95437");
    }

    #[test]
    fn test_filesystem_p2() {
        let input="$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(DaySeven{}.part_two(&input), "Size to be deleted: 24933642");
    }
}
