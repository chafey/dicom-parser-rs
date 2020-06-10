use crate::attribute::Attribute;

#[derive(PartialEq)]
pub enum Control {
    Element, // skip to next element
    Data,    // decode element data
    Stop,    // stop parsing
}

pub trait Handler {
    fn element(&mut self, _attribute: &Attribute) -> Control {
        Control::Data
    }
    fn data(&mut self, _attribute: &Attribute, _data: &[u8]) {}
    fn start_sequence_item(&mut self, _attribute: &Attribute) {}
    fn end_sequence_item(&mut self, _attribute: &Attribute) {}
    fn basic_offset_table(&mut self, _attribute: &Attribute, _data: &[u8]) -> Control {
        Control::Data
    }
    fn pixel_data_fragment(&mut self, _attribute: &Attribute, _data: &[u8]) -> Control {
        Control::Data
    }
}
