use std::convert::TryInto;

fn le_u16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]].try_into().unwrap())
}

fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
}

fn length_is_u32(bytes: &[u8]) -> bool {
    if (bytes[0] == b'O' && bytes[1] == b'B') ||
        (bytes[0] == b'O' && bytes[1] == b'W') ||
        (bytes[0] == b'S' && bytes[1] == b'Q') ||
        (bytes[0] == b'O' && bytes[1] == b'F') ||
        (bytes[0] == b'U' && bytes[1] == b'T') ||
        (bytes[0] == b'U' && bytes[1] == b'N') {
            return true
        } else {
            return false
        }
}

#[derive(Debug)]
pub struct Attribute {
    pub group: u16,
    pub element: u16,
    pub vr: [u8;2],
    pub length: usize,
    pub data_position: usize
}

impl Attribute {
    pub fn ele(bytes: &[u8]) -> Attribute {
        let mut attr = Attribute{
            group: le_u16(&bytes[0..=1]),
            element: le_u16(&bytes[2..=3]),
            vr: [bytes[4], bytes[5]],
            length: 0,
            data_position: 0
        };

        if length_is_u32(&bytes[4..]) {
            attr.length = le_u32(&bytes[8..]) as usize;
            attr.data_position = 12;
        } else {
            attr.length = le_u16(&bytes[6..]) as usize;
            attr.data_position = 8;
        }

        attr 
    }

    pub fn ile(bytes: &[u8]) -> Attribute {
        let attr = Attribute{
            group: le_u16(&bytes[0..=1]),
            element: le_u16(&bytes[2..=3]),
            vr: [b'U', b'N'],
            length: le_u32(&bytes[4..]) as usize,
            data_position: 8
        };
        attr
    }
}

#[cfg(test)]
mod tests {
    use super::Attribute;

    #[test]
    fn ele_16_len() {
        let bytes = vec![8,0, 8,0, 0x43,0x53, 0x16, 00];
        let attr = Attribute::ele(&bytes);
        assert_eq!(attr.group, 8);
        assert_eq!(attr.element, 8);
        assert_eq!(attr.vr[0], b'C');
        assert_eq!(attr.vr[1], b'S');
        assert_eq!(attr.length, 22);
    }

    #[test]
    fn ele_32_len() {
        let bytes = vec![2,0, 1,0, 0x4F,0x42, 0,0, 2,0,0,0];
        let attr = Attribute::ele(&bytes);
        assert_eq!(attr.group, 2);
        assert_eq!(attr.element, 1);
        assert_eq!(attr.vr[0], b'O');
        assert_eq!(attr.vr[1], b'B');
        assert_eq!(attr.length, 2);
    }

    #[test]
    fn ile() {
        let bytes = vec![8,0, 8,0, 0x16,0,0,0];
        let attr = Attribute::ile(&bytes);
        assert_eq!(attr.group, 8);
        assert_eq!(attr.element, 8);
        assert_eq!(attr.length, 22);
    }

}