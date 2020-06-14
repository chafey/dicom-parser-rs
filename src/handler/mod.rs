use crate::attribute::Attribute;

#[derive(PartialEq)]
pub enum Control {
    Continue, // continue (decode the element's data)
    Filter,   // filter this element (skip to the next element and ignore its data)
    Stop,     // stop parsing
}

pub trait Handler {
    fn element(&mut self, _attribute: &Attribute) -> Control {
        Control::Continue
    }
    fn data(&mut self, _attribute: &Attribute, _data: &[u8]) {}
    fn start_sequence(&mut self, _attribute: &Attribute) {}
    fn start_sequence_item(&mut self, _attribute: &Attribute) {}
    fn end_sequence_item(&mut self, _attribute: &Attribute) {}
    fn end_sequence(&mut self, _attribute: &Attribute) {}
    fn basic_offset_table(&mut self, _attribute: &Attribute, _data: &[u8]) -> Control {
        Control::Continue
    }
    fn pixel_data_fragment(&mut self, _attribute: &Attribute, _data: &[u8]) -> Control {
        Control::Continue
    }
}

pub mod data_set;
pub mod filter;
pub mod stop;
