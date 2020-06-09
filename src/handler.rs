use crate::attribute::Attribute;

#[derive(PartialEq)]
pub enum Control {
    Element, // skip to next element
    Data,    // decode element data
    Stop,    // stop parsing
}

pub trait Handler {
    fn element(&mut self, attribute: &Attribute) -> Control;
    fn data(&mut self, attribute: &Attribute, data: &[u8]);
    fn start_sequence_item(&mut self, attribute: &Attribute);
    fn end_sequence_item(&mut self, attribute: &Attribute);
}
