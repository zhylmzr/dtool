#![feature(with_options)]

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;

use strings::{find_words_on_u8, open_file};

static MIN_LENGTH: usize = 6;

fn get_strings_from_file(filename: &str) -> Vec<String> {
    let buff = open_file(filename);
    find_words_on_u8(buff, MIN_LENGTH)
}

fn get_strings_from_dir(path: &str) {
    let paths = fs::read_dir(path).unwrap();
    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.to_owned().add(".txt"))
        .unwrap();

    for path in paths {
        let p = path.unwrap().path();
        if p.is_file() {
            let words = get_strings_from_file(&p.display().to_string());
            fd.write_all(words.join("\n").as_bytes()).unwrap();
        }
    }
}

fn main() {
    let dir = "output";
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            continue;
        }
        let subdir = path.display().to_string();
        if subdir.split("/").filter(|&a| a.starts_with(".")).count() > 0 {
            continue;
        }
        get_strings_from_dir(&subdir);
    }
}
