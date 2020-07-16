use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct Wdf {
    magic: u32,
    version: u32,
    file_number: u32,
    list_offset: u32,

    list: Vec<Entity>,
    reader: BufReader<File>,
}

impl Wdf {
    pub fn new(filename: &str) -> Self {
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

        Self {
            magic,
            version,
            file_number,
            list_offset,
            list,
            reader,
        }
    }

    pub fn get_file_number(&self) -> u32 {
        return self.file_number;
    }

    pub fn extra_all(&mut self, output: &str) -> Result<(), Error> {
        let path = Path::new(output);
        if !path.exists() {
            fs::create_dir_all(path.to_str().unwrap());
        }
        for entity in &self.list {
            let filename = entity.uid.to_string() + "." + &entity.get_magic(&mut self.reader).unwrap();
            entity.save(&mut self.reader, path.join(filename).to_str().unwrap())?;
        }
        Ok(())
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
        if buff.len() >0 && buff.is_extension() {
            Ok(String::from(String::from_utf8_lossy(buff)))
        } else {
            Ok("unknown".to_owned())
        }
    }

    fn save(&self, reader: &mut BufReader<File>, filename: &str) -> Result<(), Error> {
        let mut f = match File::create(filename) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("Create file failed {}", filename);
                return Ok(());
            }
        };
        reader.seek(SeekFrom::Start(self.offset as u64))?;
        let mut buff = vec![0u8; self.size as usize];
        reader.read(buff.as_mut())?;
        f.write_all(buff.as_mut())?;
        Ok(())
    }
}

trait Extension {
    fn is_extension(&self) -> bool;
}

impl Extension for [u8] {
    fn is_extension(&self) -> bool {
        self.iter().all(|&b| { (b'0' <= b && b <= b'9') || (b'a' <= b && b <= b'z') || (b'A' <= b && b <= b'Z') || b == b'_' })
    }
}

#[cfg(test)]
mod tests {
    use crate::Wdf;

    #[test]
    fn test_wdf_number() {
        let wdf = Wdf::new("character.wdf");
        assert_eq!(wdf.get_file_number(), 6851);
    }

    #[test]
    fn test_entity_magic() {
        let mut wdf = Wdf::new("character.wdf");
        wdf.extra_all("output/character").unwrap();
    }
}
