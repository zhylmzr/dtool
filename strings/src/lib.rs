use std::fs::File;
use std::io::Read;
use std::str;

use encoding_rs::GBK;

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

trait ASCII {
    fn is_filter_character(&self) -> bool;
}

impl ASCII for u8 {
    fn is_filter_character(&self) -> bool {
        (*self >= 0x2e && *self <= 0x39) || // 0-9 . /
            (*self >= 0x41 && *self <= 0x5a) || // A-Z
            (*self >= 0x61 && *self <= 0x7a) || // a-z
            (*self == 0x5c || *self == 0x5f) // \ _
    }
}

pub fn open_file(filename: &str) -> Vec<u8> {
    let mut fd = File::open(filename).unwrap();
    let mut ret = vec![];

    fd.read_to_end(&mut ret).unwrap();
    ret
}

pub fn find_words_on_u8(buff: Vec<u8>, min_length: usize) -> Vec<String> {
    let mut ret = vec![];
    let mut pair = vec![];
    let mut is_gbk2312_start = false;

    let deal_end_word = |is_gbk2312_start: &mut bool, pair: &mut Vec<u8>, ret: &mut Vec<String>| {
        // We have to remove character if isn't gbk2312
        if *is_gbk2312_start && !pair.is_empty() {
            pair.remove(pair.len() - 1);
            *is_gbk2312_start = false;
        }

        if !pair.is_empty() {
            if pair.len() >= min_length {
                let s = GBK.decode(pair.as_slice()).0;
                ret.push(String::from(s));
            }
            pair.clear();
        }
    };

    for ch in buff {
        if ch.is_filter_character() && !is_gbk2312_start {
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
            deal_end_word(&mut is_gbk2312_start, &mut pair, &mut ret);
        }
    }

    // Tak the rest, if any
    deal_end_word(&mut is_gbk2312_start, &mut pair, &mut ret);

    ret
}

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