use std::cmp::Ordering;
use std::collections::HashMap;

use crate::input;
use crate::print;

/**
 * File, storing its name and size
 */
#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: &str, size: usize) -> File {
        File {
            name: name.to_string(),
            size,
        }
    }
}

/**
 * Directory, containing files and subdirectories
 */
#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
    size: usize,
}

impl Directory {
    pub fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            files: HashMap::new(),
            directories: HashMap::new(),
            size: 0,
        }
    }

    pub fn update_size(&mut self) -> usize {
        self.size = self.files.values().map(|f| f.size).sum();
        for dir in self.directories.values_mut() {
            self.size += dir.update_size()
        }
        self.size
    }
}

impl Ord for Directory {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.size, &self.name).cmp(&(other.size, &other.name))
    }
}

impl PartialOrd for Directory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        (self.size, &self.name) == (other.size, &other.name)
    }
}

impl Eq for Directory {}

/**
 * Commands
 */
#[derive(Debug)]
enum Command {
    Ls(Vec<File>, Vec<Directory>),
    Cd(Vec<String>),
}

/**
 * Tracks a working directory as a vector of path segments
 */
struct WorkingDirectory {
    segments: Vec<String>,
}

impl WorkingDirectory {
    pub fn new() -> WorkingDirectory {
        WorkingDirectory { segments: vec![] }
    }
    pub fn cd(&mut self, path: &str) {
        match path {
            "/" => self.segments.clear(),
            ".." => {
                self.segments.pop();
            }
            dir => self.segments.push(dir.to_string()),
        }
    }
    pub fn pwd(&self) -> Vec<String> {
        self.segments.clone()
    }
}

/**
 * Creates a list of Commands from stdin (puzzle input)
 */
fn parse_stdin(stdin: &[String]) -> Vec<Command> {
    let mut wd = WorkingDirectory::new();
    let mut cmds: Vec<Command> = vec![];

    for line in stdin {
        let mut parts = line.split_ascii_whitespace();

        match parts.next() {
            // Determine if this line is a command
            Some("$") => {
                // Determine which command it is
                match parts.next() {
                    // Handle `change directory`
                    Some("cd") => {
                        if let Some(dir) = parts.next() {
                            wd.cd(dir);
                            cmds.push(Command::Cd(wd.pwd()));
                        } else {
                            continue;
                        }
                    }
                    // Handle `list`
                    Some("ls") => {
                        cmds.push(Command::Ls(vec![], vec![]));
                    }
                    _ => continue,
                }
            }
            // This line isn't a command, so it's info about a file or directory
            Some(file_or_dir) => {
                // We should be able to borrow the last command as mutable
                // Last command *should* always be `list`
                if let Some(Command::Ls(files, dirs)) = cmds.last_mut() {
                    if file_or_dir == "dir" {
                        // Is directory
                        if let Some(dir_name) = parts.next() {
                            dirs.push(Directory::new(dir_name));
                        }
                    } else {
                        // Is file
                        if let Ok(size) = file_or_dir.parse::<usize>() {
                            if let Some(file_name) = parts.next() {
                                files.push(File::new(file_name, size));
                            }
                        }
                    }
                }
            }
            None => continue,
        };
    }

    cmds
}

/**
 * Builds a filesystem from stdin (puzzle input)
*/
fn build_filesystem(stdin: &[String]) -> Directory {
    let cmds = parse_stdin(stdin);
    let mut root = Directory::new("root");
    let mut wd_ptr: *mut Directory = &mut root;

    for cmd in cmds {
        match cmd {
            Command::Cd(path) => {
                // Attempt to traverse this path
                wd_ptr = &mut root;
                for seg in path {
                    unsafe {
                        if let Some(dir) = (*wd_ptr).directories.get_mut(&seg) {
                            wd_ptr = dir
                        }
                    }
                }
            }
            Command::Ls(files, dirs) => unsafe {
                let wd = &mut *wd_ptr;
                for file in files {
                    wd.files.insert(file.name.to_string(), file);
                }
                for dir in dirs {
                    wd.directories.insert(dir.name.to_string(), dir);
                }
            },
        };
    }

    root.update_size();
    root
}

// PART 1
// Find and sum all directories with a total size of < 100_000

const SIZE_THRESHOLD: usize = 100_000;

fn sum_dirs_recursive(dir: &Directory) -> usize {
    let to_add = if dir.size <= SIZE_THRESHOLD {
        dir.size
    } else {
        0
    };
    to_add
        + dir
            .directories
            .values()
            .map(sum_dirs_recursive)
            .sum::<usize>()
}

fn sum_filesystem(stdin: &[String]) -> usize {
    let fs = build_filesystem(stdin);
    sum_dirs_recursive(&fs)
}

pub fn part1() {
    print::intro(7, 1);

    let stdin = input::day_input::<String>(7);

    let sum_test = sum_filesystem(&stdin.test);
    let sum_real = sum_filesystem(&stdin.real);

    print::answer_with_test(sum_real, sum_test);
}

// PART 2
// Find the smallest directory that could be deleted to free up enough space

const TOTAL_SPACE: usize = 70_000_000;
const FREE_SPACE_TARGET: usize = 30_000_000;

fn list_dirs(dir: &Directory) -> Vec<&Directory> {
    let mut dirs = dir
        .directories
        .values()
        .flat_map(list_dirs)
        .collect::<Vec<&Directory>>();
    dirs.push(dir);
    dirs
}

fn find_delete_candidate(stdin: &[String]) -> Option<usize> {
    let fs = build_filesystem(stdin);
    let mut dirs = list_dirs(&fs);
    dirs.sort();

    let available_space = TOTAL_SPACE - fs.size;
    for dir in dirs {
        if available_space + dir.size >= FREE_SPACE_TARGET {
            return Some(dir.size);
        }
    }

    None
}

pub fn part2() {
    print::intro(7, 2);

    let stdin = input::day_input::<String>(7);

    let size_test = find_delete_candidate(&stdin.test).expect("No test solution");
    let size_real = find_delete_candidate(&stdin.real).expect("No real solution");

    print::answer_with_test(size_real, size_test);
}
