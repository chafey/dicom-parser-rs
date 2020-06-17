use crate::attribute::Attribute;

#[derive(PartialEq)]
pub enum HandlerResult {
    Continue, // continue (decode the element's data)
    Cancel,   // stop parsing
}

pub trait Handler {
    fn attribute(
        &mut self,
        _attribute: &Attribute,
        _position: usize,
        _data_offset: usize,
    ) -> HandlerResult {
        HandlerResult::Continue
    }
    fn data(&mut self, _attribute: &Attribute, _data: &[u8]) {}
    fn start_sequence(&mut self, _attribute: &Attribute) {}
    fn start_sequence_item(&mut self, _attribute: &Attribute) {}
    fn end_sequence_item(&mut self, _attribute: &Attribute) {}
    fn end_sequence(&mut self, _attribute: &Attribute) {}
    fn basic_offset_table(&mut self, _attribute: &Attribute, _data: &[u8]) -> HandlerResult {
        HandlerResult::Continue
    }
    fn pixel_data_fragment(&mut self, _attribute: &Attribute, _data: &[u8]) -> HandlerResult {
        HandlerResult::Continue
    }
}

pub mod cancel;
pub mod data_set;
