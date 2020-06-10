use crate::attribute::Attribute;
use crate::parser::handler::{Control, Handler};

pub type FilterFN = fn(&Attribute) -> bool;

pub struct FilterHandler<'t> {
    pub filter_fn: FilterFN,
    pub handler: &'t mut dyn Handler,
}

impl Handler for FilterHandler<'_> {
    fn element(&mut self, attribute: &Attribute) -> Control {
        if (self.filter_fn)(&attribute) {
            return Control::Element;
        }
        self.handler.element(&attribute)
    }
    fn data(&mut self, attribute: &Attribute, data: &[u8]) {
        self.handler.data(&attribute, data)
    }
    fn start_sequence_item(&mut self, attribute: &Attribute) {
        self.handler.start_sequence_item(&attribute)
    }
    fn end_sequence_item(&mut self, attribute: &Attribute) {
        self.handler.end_sequence_item(&attribute)
    }
    fn basic_offset_table(&mut self, attribute: &Attribute, data: &[u8]) -> Control {
        self.handler.basic_offset_table(&attribute, data)
    }
    fn pixel_data_fragment(&mut self, attribute: &Attribute, data: &[u8]) -> Control {
        self.handler.pixel_data_fragment(&attribute, data)
    }
}
