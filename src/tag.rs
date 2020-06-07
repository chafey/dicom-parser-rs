use crate::byte_parser::le_u16;
use std::fmt;

#[derive(Eq)]
pub struct Tag {
    group: u16,
    element: u16
}

impl Tag {
    pub fn new(group: u16, element: u16) -> Tag {
        Tag {
            group,
            element
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Tag {
        Tag {
            group: le_u16(&bytes[0..=1]),
            element: le_u16(&bytes[2..=3]),
        }
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({:04X},{:04X})", self.group, self.element))
    }
}

impl PartialEq for Tag {
    fn eq(&self, right: &Tag) -> bool {
        self.group == right.group &&
        self.element == right.element
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn new() {
        let tag = Tag::new(8,10);
        assert_eq!(tag.group, 8);
        assert_eq!(tag.element, 10);
    }

    #[test]
    fn from_bytes() {
        let tag = Tag::from_bytes(&[8,0,10,0]);
        assert_eq!(tag.group, 8);
        assert_eq!(tag.element, 10);
    }
}