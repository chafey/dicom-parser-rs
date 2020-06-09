use crate::tag::Tag;
use crate::vr::VR;

#[derive(Debug, Clone, Copy)]
pub struct Attribute {
    pub tag: Tag,
    pub vr: Option<VR>,
    pub length: usize,
    pub had_unknown_length: bool
}

pub type AttributeFN = fn(&[u8]) -> Attribute;

