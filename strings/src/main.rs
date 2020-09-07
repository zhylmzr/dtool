#![feature(with_options)]

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::path::Path;

use strings::{find_words_on_u8, open_file};

static MIN_LENGTH: usize = 3;

fn get_strings_from_file(filename: &str) -> Vec<String> {
    let buff = open_file(filename);
    find_words_on_u8(buff, MIN_LENGTH)
}

fn get_strings_from_dir(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut result = vec![];

    for path in paths {
        let p = path.unwrap().path();
        if p.is_file() {
            let words = get_strings_from_file(&p.display().to_string());
            result = vec![result, words, '\n'].concat();
        }
    }

    result
}

fn save_strings(filename: &str, strings: Vec<String>) {
    let mut fd = OpenOptions::new().write(true).create(true).open(filename).unwrap();
    let content = strings.join("\n");
    fd.write_all(content.as_bytes());
}

fn main() {
    let dir = "output";
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() { continue; }
        let subdir = path.display().to_string();
        if subdir.split("/").filter(|&a| { a.starts_with(".") }).count() > 0 {
            continue;
        }
        let res = get_strings_from_dir(&subdir);
        let path = Path::new(dir).join(subdir.add(".text"));
        save_strings(path.to_str().unwrap(), res);
    }
}