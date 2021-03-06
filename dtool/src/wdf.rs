#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::{collections::HashMap, convert::TryInto};

use crate::text;

const TEXT_EXTENTIONS: [&'static str; 3] = [".txt", ".xml", ".ini"];
const TEXT_EXCLUDES: [&'static str; 1] = ["caption.xml"];

pub struct Wdf {
    magic: u32,
    version: u32,
    file_number: u32,
    list_offset: u32,

    list: Vec<Entity>,
    reader: BufReader<File>,
    pkg_name: String,

    name_list: HashMap<u32, String>,
}

impl Wdf {
    pub fn new(filename: &str) -> Self {
        Wdf::init(filename)
    }

    pub fn get_file_number(&self) -> u32 {
        return self.file_number;
    }

    fn extra(&mut self, output: &str) -> Result<(), Error> {
        let path = Path::new(output).join(self.pkg_name.as_str());
        for entity in &self.list {
            let filename = if self.name_list.contains_key(&entity.uid) {
                let path = Path::new(output).join(self.name_list.get(&entity.uid).unwrap());
                if !path.parent().unwrap().exists() {
                    fs::create_dir_all(path.parent().unwrap().to_str().unwrap()).unwrap();
                }
                path.to_str().unwrap().to_string()
            } else {
                let filename = format!(
                    "{}.{}",
                    entity.uid.to_string(),
                    entity.get_magic(&mut self.reader).unwrap()
                );
                let path = path.join("unknown");
                if !path.exists() {
                    fs::create_dir_all(path.to_str().unwrap()).unwrap();
                }
                path.join(filename).to_str().unwrap().to_string()
            };

            entity.save(&mut self.reader, &filename)?;
        }
        Ok(())
    }

    pub fn extra_all(&mut self, output: &str) -> Result<(), Error> {
        self.extra(output)
    }

    pub fn extra_all_with_hash(&mut self, output: &str, hash_path: &str) -> Result<(), Error> {
        let uid_lst = fs::read_to_string(hash_path).unwrap();
        let mut contents = String::new();
        uid_lst.chars().enumerate().for_each(|(_, c)| {
            if c != '\r' {
                contents.push(c);
            }
        });
        let contents: Vec<_> = contents.split("\n").filter(|&s| s != "").collect();

        for line in contents {
            let line: Vec<_> = line.split("|").collect();
            let filename = line[0].replace("\\", "/");
            let uid = line[1].parse::<u32>().unwrap();
            self.name_list.insert(uid, filename.to_string());
        }

        self.extra(output)
    }

    fn init(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::new(f);

        // read header
        let mut buff = [0u8; 4 * 4];
        reader.read(&mut buff).unwrap();
        let magic = u32::from_le_bytes(buff[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(buff[4..8].try_into().unwrap());
        let file_number = u32::from_le_bytes(buff[8..12].try_into().unwrap());
        let list_offset = u32::from_le_bytes(buff[12..16].try_into().unwrap());

        // read lists
        let mut buff = [0u8; 4 * 4];
        let mut list = Vec::new();
        reader.seek(SeekFrom::Start(list_offset as u64)).unwrap();
        for _ in 0..file_number {
            reader.read(&mut buff).unwrap();
            let entity = Entity::new(&buff);
            list.push(entity);
        }

        let path = Path::new(filename);

        let pkg_name = path.file_stem().unwrap().to_str().unwrap().to_owned();

        Self {
            magic,
            version,
            file_number,
            list_offset,
            list,
            reader,
            pkg_name,
            name_list: HashMap::new(),
        }
    }
}

struct Entity {
    uid: u32,
    offset: u32,
    size: u32,
    space: u32,
}

impl Entity {
    fn new(data: &[u8]) -> Self {
        Self {
            uid: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            offset: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            size: u32::from_le_bytes(data[8..12].try_into().unwrap()),
            space: u32::from_le_bytes(data[12..16].try_into().unwrap()),
        }
    }

    fn get_magic(&self, reader: &mut BufReader<File>) -> Result<String, Error> {
        let mut buff = [0u8; 4];
        reader.seek(SeekFrom::Start(self.offset as u64))?;
        reader.read(&mut buff)?;

        let buff = buff.split(|&a| a == 0).next().unwrap();
        if buff.len() > 0 && buff.is_extension() {
            Ok(String::from(String::from_utf8_lossy(buff)))
        } else {
            Ok("unknown".to_owned())
        }
    }

    fn save(&self, reader: &mut BufReader<File>, filename: &str) -> Result<(), Error> {
        let mut f = match File::create(filename) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Create file failed {}", filename);
                return Ok(());
            }
        };
        reader.seek(SeekFrom::Start(self.offset as u64))?;
        let mut buff = vec![0u8; self.size as usize];
        reader.read(buff.as_mut())?;
        if let Some(_) = TEXT_EXTENTIONS.iter().find(|&&a| filename.ends_with(a)) {
            if let None = TEXT_EXCLUDES.iter().find(|&&a| filename.ends_with(a)) {
                buff = Vec::from(text::Text::decode(buff.as_mut()));
            }
        }
        f.write_all(buff.as_mut())?;
        Ok(())
    }
}

trait Extension {
    fn is_extension(&self) -> bool;
}

impl Extension for [u8] {
    fn is_extension(&self) -> bool {
        self.iter().all(|&b| {
            (b'0' <= b && b <= b'9')
                || (b'a' <= b && b <= b'z')
                || (b'A' <= b && b <= b'Z')
                || b == b'_'
        })
    }
}
