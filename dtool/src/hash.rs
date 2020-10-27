#![allow(dead_code)]

use crate::uint32::Uint32;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

const WDF_LIST: [&str; 8] = [
    "character",
    "helper",
    "fx",
    "interface",
    "object",
    "setting",
    "tile",
    "map",
];

#[rustfmt::skip]
fn mix(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
    let mut a = Uint32::from(a);
    let mut b = Uint32::from(b);
    let mut c = Uint32::from(c);

    a -= b; a -= c; a ^= c>>13;
    b -= c; b -= a; b ^= a<<8;
    c -= a; c -= b; c ^= b>>13;
    a -= b; a -= c; a ^= c>>12; 
    b -= c; b -= a; b ^= a<<16;
    c -= a; c -= b; c ^= b>>5;
    a -= b; a -= c; a ^= c>>3; 
    b -= c; b -= a; b ^= a<<10;
    c -= a; c -= b; c ^= b>>15;

    return (a.0, b.0, c.0);
}

fn hash(buff: &[u8]) -> u32 {
    let length = buff.len();
    let mut len = length;
    let mut a = Uint32::from(0x9e3779b9u32);
    let mut b = Uint32::from(0x9e3779b9u32);
    let mut c = Uint32::from(0);
    let mut k = Vec::from(buff);

    while len >= 12 {
        let (k0, k1, k2, k3, k4, k5, k6, k7, k8, k9, k10, k11) = (
            Uint32::from(k[0]),
            Uint32::from(k[1]),
            Uint32::from(k[2]),
            Uint32::from(k[3]),
            Uint32::from(k[4]),
            Uint32::from(k[5]),
            Uint32::from(k[6]),
            Uint32::from(k[7]),
            Uint32::from(k[8]),
            Uint32::from(k[9]),
            Uint32::from(k[10]),
            Uint32::from(k[11]),
        );

        a += k0 + (k1 << 8) + (k2 << 16) + (k3 << 24);
        b += k4 + (k5 << 8) + (k6 << 16) + (k7 << 24);
        c += k8 + (k9 << 8) + (k10 << 16) + (k11 << 24);
        let r = mix(a.0, b.0, c.0);
        a = Uint32::from(r.0);
        b = Uint32::from(r.1);
        c = Uint32::from(r.2);
        k = Vec::from(k.split_at(12).1);
        len -= 12;
    }
    c += length;

    while len >= 1 {
        let v = Uint32::from(k[len - 1]);
        let p = match len {
            9 | 6 | 2 => 8,
            10 | 7 | 3 => 16,
            11 | 8 | 4 => 24,
            _ => 0,
        };

        match len {
            9..=11 => c += v << p,
            5..=8 => b += v << p,
            _ => a += v << p,
        }

        len -= 1;
    }

    let (_, _, c) = mix(a.0, b.0, c.0);
    return c;
}

pub fn string_id(full_name: &[u8]) -> u32 {
    let mut res = vec![];
    let full_name = Vec::from(full_name);
    let mut pkg_name = vec![];
    let mut filename = vec![];

    if let Some((pos, _)) = full_name
        .iter()
        .enumerate()
        .find(|(_, &a)| a == b'/')
        .or(full_name.iter().enumerate().find(|(_, &a)| a == b'\\'))
    {
        let full = &full_name[..];
        pkg_name.extend_from_slice(&full[..pos]);
        filename.extend_from_slice(&full[pos + 1..]);
    }

    let full_name = match WDF_LIST
        .iter()
        .find(|&&a| a.as_bytes() == pkg_name.to_ascii_lowercase())
    {
        Some(_) => filename,
        None => full_name,
    };

    full_name.iter().for_each(|&v| {
        let ch = v as char;
        if ch == '\\' {
            res.push(b'/')
        } else if ch.is_uppercase() {
            res.push(v.to_ascii_lowercase() as u8)
        } else {
            res.push(v)
        }
    });

    return hash(&res);
}

fn calc_hash_from_lst(lst_path: &str, out_path: &str) {
    let content = fs::read(lst_path).unwrap();
    // 兼容CRLF换行
    let content: Vec<_> = content.into_iter().filter(|&a| a != b'\r').collect();
    let content: Vec<_> = content.split(|&a| a == b'\n').collect();

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(out_path)
        .unwrap();

    for (idx, filename) in content.iter().enumerate() {
        if filename.len() == 0 {
            continue;
        }
        let uid = string_id(filename);
        f.write_all(filename).unwrap();
        f.write_all(" ".as_bytes()).unwrap();
        f.write_all(uid.to_string().as_bytes()).unwrap();
        if idx != content.len() - 1 {
            f.write_all("\n".as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_id() {
        assert_eq!(string_id("TILE\\ANI\\BH_MLOU_03.ara".as_bytes()), 770704020);
        assert_eq!(
            string_id("character/ani/2011znq001.ara".as_bytes()),
            3954770787
        );
    }
}
