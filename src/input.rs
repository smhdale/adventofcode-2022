use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct RawInput {
    pub test: String,
    pub real: String,
}

pub struct DayInput<T> {
    pub test: Vec<T>,
    pub real: Vec<T>,
}

pub struct DayInputGrouped<T> {
    pub test: Vec<Vec<T>>,
    pub real: Vec<Vec<T>>,
}

fn raw_from_file(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("File not found");
    let mut data = String::new();
    BufReader::new(file)
        .read_to_string(&mut data)
        .expect("Failed to read file");
    data
}

fn lines_from_file<T>(filename: impl AsRef<Path>) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file).lines();
    let mut data = vec![];

    for line in lines {
        let value = line.unwrap();
        if !value.is_empty() {
            data.push(value.parse::<T>().unwrap());
        }
    }

    data
}

fn lines_from_file_grouped<T>(filename: impl AsRef<Path>) -> Vec<Vec<T>>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut data: Vec<Vec<T>> = vec![];
    let mut group: Vec<T> = vec![];

    for line in lines {
        let value = line.unwrap();
        if !value.is_empty() {
            group.push(value.parse::<T>().unwrap());
        } else if !group.is_empty() {
            data.push(group.to_vec());
            group.clear();
        }
    }

    // Ensure final group is pushed if no newline at end of file
    if !group.is_empty() {
        data.push(group.to_vec());
    }

    data
}

fn get_day_file(day: u8, file: &str) -> PathBuf {
    let mut path = env::current_dir().expect("Unable to determine working directory");
    let day = format!("day{}", day);

    path.push("src/puzzles");
    path.push(day);
    path.push(file);

    path
}

pub fn day_input_raw(day: u8) -> RawInput {
    let test = get_day_file(day, "input_test.txt");
    let real = get_day_file(day, "input.txt");
    RawInput {
        test: raw_from_file(&test),
        real: raw_from_file(&real),
    }
}

pub fn day_input<T: FromStr>(day: u8) -> DayInput<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let test = get_day_file(day, "input_test.txt");
    let real = get_day_file(day, "input.txt");
    DayInput {
        test: lines_from_file::<T>(&test),
        real: lines_from_file::<T>(&real),
    }
}

pub fn day_input_grouped<T: FromStr>(day: u8) -> DayInputGrouped<T>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Debug,
{
    let test = get_day_file(day, "input_test.txt");
    let real = get_day_file(day, "input.txt");
    DayInputGrouped {
        test: lines_from_file_grouped::<T>(&test),
        real: lines_from_file_grouped::<T>(&real),
    }
}
