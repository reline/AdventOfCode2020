use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::PathBuf;

pub fn file_to_int_array(path: PathBuf) -> Vec<i32> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|s| {
            s.and_then(|v| {
                v.parse::<i32>()
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
            })
        })
        // ignore lines that fail
        .filter_map(Result::ok)
        .collect::<Vec<i32>>()
}

pub fn file_to_lines(path: PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
}

pub fn file_to_string(path: PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    return string;
}
