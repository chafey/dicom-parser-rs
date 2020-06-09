use std::convert::TryInto;

pub trait ByteParser {
    fn u16(bytes: &[u8]) -> u16;
    fn u32(bytes: &[u8]) -> u32;
}

#[allow(dead_code)]
pub struct LittleEndianByteParser {}

impl ByteParser for LittleEndianByteParser {
    fn u16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes([bytes[0], bytes[1]].try_into().unwrap())
    }

    fn u32(bytes: &[u8]) -> u32 {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
    }
}

#[allow(dead_code)]
pub struct BigEndianByteParser {}

impl ByteParser for BigEndianByteParser {
    fn u16(bytes: &[u8]) -> u16 {
        u16::from_be_bytes([bytes[0], bytes[1]].try_into().unwrap())
    }

    fn u32(bytes: &[u8]) -> u32 {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::le_u32;

    #[test]
    fn read_success() {
        let bytes = vec![0xfe,0xff,0x00,0xe0];
        let value = le_u32(&bytes);
        println!("value = {}", value);
        assert_eq!(value, xfffee000);
    }
}*/
