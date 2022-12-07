use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;

struct File {
    name: String, 
    size: u32,
}

#[derive(Default)]
struct Directory {
    subdirs: Vec<PathBuf>,
    files: Vec<File>,
    size: Option<u32>,
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

fn print_sum(sizes: &HashMap<PathBuf, u32>) {
    //@@@ this would be better with a filter + reduce
    let mut total = 0;
    for size in sizes.values() {
        if *size < 100000 {
            total += size;
        }
    }
    println!("Sum of sizes below 100000 {total}")
}

fn print_size_to_delete(sizes: &HashMap<PathBuf, u32>) {
    let total: u32 = sizes.values().sum();
    let space_to_free = 300000 - total;
    println!("{space_to_free}");
    let size_vec: Vec<&u32> = sizes.values().collect();

    for value in size_vec {
        if *value > space_to_free {
            println!("Size to be deleted: {value}");
            return
        }
    }
}
        
fn change_dir(dirs: &mut HashMap<PathBuf, Directory>, current_path: &PathBuf, relative_path: &str) -> PathBuf {
    let path: PathBuf = match relative_path {
        "/" => Path::new("/").to_path_buf(),
        ".." => current_path.parent().unwrap().to_path_buf(),
        _ => current_path.join(relative_path),
    };
    let str = path.to_str().unwrap();
    println!("adding {str}");
    dirs.entry(path.clone()).or_default();
    path
}

fn add_file(dirs: &mut HashMap<PathBuf, Directory>, dir_path: &PathBuf, file: &str)  {
    dbg!(file);
    let dir: &mut Directory = dirs.get_mut(dir_path).unwrap();
    let re: Regex = Regex::new(r"^(?P<size>[0-9]+)\s(?P<name>[a-z.]+)\s*$").unwrap();
    //lazy_static! {
    //    static ref RE: Regex = Regex::new(r"^(?P<size>[0-9]+)\s(?P<name>[a-z.]+)\s*$").unwrap();
    //}
    
    let cap = re.captures(file.trim()).unwrap();
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

fn main() {
    let input = "$ cd /
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

    let mut dirs: HashMap<PathBuf, Directory> = HashMap::new();
    let mut current_filepath = Path::new("/").to_path_buf();
    for output in input.lines() {
        if output.starts_with("$ cd") {
            println!("cding: {output}");
            current_filepath = change_dir(&mut dirs, &current_filepath, output[4..].trim());
        } else if output.starts_with("$ ls") {
            println!("lsing: {output}");
            continue
        } else {
            if output.starts_with("dir") {
                println!("adding subdir: {output}");
                add_subdir(&mut dirs, &current_filepath, &output[4..]);
            } else {
                println!("adding file: {output}");
                add_file(&mut dirs, &current_filepath, output);
            }
        }
    }
    
    let sizes = get_sizes(&dirs);
    print_sum(&sizes);
    print_size_to_delete(&sizes);
}
