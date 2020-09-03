use std::fs::File;
use std::io::Read;
use std::str;

use encoding_rs::GBK;

#[cfg(test)]
mod tests {
    use crate::find_words_on_u8;

    #[test]
    fn test_find_words_on_u8() {
        let result = find_words_on_u8(
            vec![0xC4u8, 0xE3u8, 0xBAu8, 0xC3u8, 0xCAu8, 0xC0u8,
                 0xBDu8, 0xE7u8, 0x2Cu8, 0x20u8, 0x68u8, 0x65u8,
                 0x6Cu8, 0x6Cu8, 0x6Fu8, 0x20u8, 0x77u8, 0x6Fu8,
                 0x72u8, 0x6Cu8, 0x64u8], 3);

        assert_eq!(3, result.len());
        assert_eq!("你好世界,", result[0]);
        assert_eq!("hello", result[1]);
        assert_eq!("world", result[2]);
    }
}

// 0xb0a1-0xf7fe
trait GB2312 {
    fn is_gb2312_range(&self) -> bool;
    fn is_gb2312_first(&self) -> bool;
}

impl GB2312 for u8 {
    fn is_gb2312_range(&self) -> bool {
        *self >= 0xa1u8 && *self <= 0xfeu8
    }

    fn is_gb2312_first(&self) -> bool {
        *self >= 0xb0u8 && *self <= 0xf7u8
    }
}


pub fn open_file(filename: &str) -> Vec<u8> {
    let mut fd = File::open(filename).unwrap();
    let mut ret = vec![];

    fd.read_to_end(&mut ret);
    ret
}

pub fn find_words_on_u8(buff: Vec<u8>, min_length: usize) -> Vec<String> {
    let mut ret = vec![];
    let mut pair = vec![];
    let mut is_gbk2312_start = false;

    for ch in buff {
        if ch.is_ascii_graphic() && !is_gbk2312_start {
            pair.push(ch);
        } else if ch.is_gb2312_range() {
            if is_gbk2312_start {
                pair.push(ch);
                is_gbk2312_start = false
            } else if ch.is_gb2312_first() {
                pair.push(ch);
                is_gbk2312_start = true
            }
        } else {
            // We have to remove character if isn't gbk2312
            if is_gbk2312_start && !pair.is_empty() {
                pair.remove(pair.len() - 1);
                is_gbk2312_start = false;
            }

            if !pair.is_empty() {
                if pair.len() >= min_length {
                    let s = GBK.decode(pair.as_slice()).0;
                    ret.push(String::from(s));
                }
                pair.clear();
            }
        }
    }

    // Tak the rest, if any
    if is_gbk2312_start && !pair.is_empty() {
        pair.remove(pair.len() - 1);
    }

    if !pair.is_empty() {
        if pair.len() >= min_length {
            let s = GBK.decode(pair.as_slice()).0;
            ret.push(String::from(s));
        }
        pair.clear();
    }

    ret
}