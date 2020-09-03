const KEY: [u8; 16] = [
    0x0C4u8, 0x6Fu8, 0x0D5u8, 0x84u8, 0x8Bu8, 0x0C0u8, 0x43u8, 0x0A8u8, 0x90u8, 0x51u8, 0x60u8,
    0x0CFu8, 0x0A7u8, 0x62u8, 0x0A4u8, 0x8Du8,
];

pub struct Text {}

impl Text {
    pub fn decode(buf: &mut [u8]) -> &[u8] {
        for (i, v) in buf.iter_mut().enumerate() {
            *v ^= KEY[i & 0xf]
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::text;

    #[test]
    fn test_decode() {
        let mut buf = [1u8, 2u8, 3u8, 4u8];
        let buf = text::Text::decode(buf.as_mut());
        println!("{:?}", buf);
    }
}
