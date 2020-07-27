use crate::encoding::Encoding;
use std::fmt;

#[derive(Default, Eq, PartialOrd, Clone, Copy)]
pub struct Tag {
    pub group: u16,
    pub element: u16,
}
impl Tag {
    pub fn new(group: u16, element: u16) -> Tag {
        Tag { group, element }
    }

    pub fn from_bytes<T: Encoding>(bytes: &[u8]) -> Tag {
        let group = T::u16(&bytes[0..2]);
        let element = T::u16(&bytes[2..4]);
        Tag { group, element }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_private(&self) -> bool {
        (self.group % 2) > 0
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({:04X},{:04X})", self.group, self.element))
    }
}

impl PartialEq for Tag {
    fn eq(&self, right: &Tag) -> bool {
        self.group == right.group && self.element == right.element
    }
}

pub static ITEM: Tag = Tag {
    group: 0xFFFE,
    element: 0xE000,
};
pub static ITEMDELIMITATIONITEM: Tag = Tag {
    group: 0xFFFE,
    element: 0xE00D,
};
pub static SEQUENCEDELIMITATIONITEM: Tag = Tag {
    group: 0xFFFE,
    element: 0xE0DD,
};

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn new() {
        let tag = Tag::new(8, 10);
        assert_eq!(tag.group, 8);
        assert_eq!(tag.element, 10);
    }

    #[test]
    fn tag_is_private_returns_false() {
        let tag = Tag::new(8, 10);
        assert_eq!(tag.is_private(), false);
    }

    #[test]
    fn tag_is_private_returns_true() {
        let tag = Tag::new(9, 10);
        assert_eq!(tag.is_private(), true);
    }
}
