use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

fn lines_from_file<T>(filename: impl AsRef<Path>) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug
{
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file).lines();
    let mut data = vec![];

    for line in lines {
        let value = line.unwrap();
        let trimmed = value.trim();
        if trimmed.len() > 0 {
            data.push(trimmed.parse::<T>().unwrap());
        }
    };

    data
}

fn lines_from_file_grouped<T>(filename: impl AsRef<Path>) -> Vec<Vec<T>>
where T: FromStr + Clone, <T as FromStr>::Err: Debug
{
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut data: Vec<Vec<T>> = vec![];
    let mut group: Vec<T> = vec![];

    for line in lines {
        let value = line.unwrap();
        let trimmed = value.trim();
        if trimmed.len() > 0 {
            group.push(trimmed.parse::<T>().unwrap());
        } else if group.len() > 0 {
            data.push(group.to_vec());
            group.clear();
        }
    };

    data
}

fn get_day_file(day: u8, file: &str) -> PathBuf {
    let mut path = env::current_dir().expect("Unable to determine working directory");
    let day = format!("day{}", day);

    path.push("src/puzzles");
    path.push(&day);
    path.push(&file);

    path
}

pub fn day_input<T: FromStr>(day: u8, file: &str) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug
{
    let path = get_day_file(day, file);
    lines_from_file::<T>(&path)
}

pub fn day_input_grouped<T: FromStr>(day: u8, file: &str) -> Vec<Vec<T>>
where T: FromStr + Clone, <T as FromStr>::Err: Debug
{
    let path = get_day_file(day, file);
    lines_from_file_grouped::<T>(&path)
}
