use std::convert::TryInto;

pub fn le_u16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]].try_into().unwrap())
}

pub fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
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
