use crate::tag::Tag;
use crate::vr::VR;

#[derive(Default, Debug, Clone, Copy)]
pub struct Attribute {
    pub tag: Tag,
    pub vr: Option<VR>,
    pub length: usize,
}
