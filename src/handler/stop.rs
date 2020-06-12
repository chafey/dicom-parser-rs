use crate::attribute::Attribute;
use crate::handler::{Control, Handler};

pub type StopFN = fn(&Attribute) -> bool;

pub struct StopHandler<'t> {
    pub stop_fn: StopFN,
    pub handler: &'t mut dyn Handler,
}

impl Handler for StopHandler<'_> {
    fn element(&mut self, attribute: &Attribute) -> Control {
        if (self.stop_fn)(&attribute) {
            return Control::Stop;
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
