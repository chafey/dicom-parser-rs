use crate::tag::Tag;
use crate::byte_parser::le_u32;

pub fn read(bytes: &[u8]) -> Result<usize, ()> {
    let tag = Tag::from_bytes(&bytes[0..4]); 
    if tag.group != 0xFFFE || tag.element != 0xE000 {
        return Err(())
    }
    let length = le_u32(&bytes[4..8]) as usize;
    Ok(length)
}

#[cfg(test)]
mod tests {
    use super::read;

    #[test]
    fn read_success() {
        let bytes = vec![0xfe,0xff,0x00,0xe0, 0x01,0x00,0x00,0x00];
        let length = read(&bytes).unwrap();
        assert_eq!(length, 1);
    }
}
