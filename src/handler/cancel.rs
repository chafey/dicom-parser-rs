use crate::attribute::Attribute;
use crate::handler::{Control, Handler};

pub type CancelFN = fn(&Attribute) -> bool;

pub struct CancelHandler<'t> {
    pub canceled: bool,
    pub handler: &'t mut dyn Handler,
    pub cancel_fn: CancelFN,
}

impl<'t> CancelHandler<'t> {
    pub fn new(handler: &'t mut dyn Handler, cancel_fn: CancelFN) -> CancelHandler<'t> {
        CancelHandler {
            canceled: false,
            handler,
            cancel_fn,
        }
    }
}

impl Handler for CancelHandler<'_> {
    fn element(&mut self, attribute: &Attribute) -> Control {
        if (self.cancel_fn)(&attribute) {
            self.canceled = true;
            return Control::Cancel;
        }
        self.handler.element(&attribute)
    }
    fn data(&mut self, attribute: &Attribute, data: &[u8]) {
        self.handler.data(&attribute, data)
    }
    fn start_sequence(&mut self, attribute: &Attribute) {
        self.handler.start_sequence(&attribute)
    }
    fn start_sequence_item(&mut self, attribute: &Attribute) {
        self.handler.start_sequence_item(&attribute)
    }
    fn end_sequence_item(&mut self, attribute: &Attribute) {
        self.handler.end_sequence_item(&attribute)
    }
    fn end_sequence(&mut self, attribute: &Attribute) {
        self.handler.end_sequence(&attribute)
    }
    fn basic_offset_table(&mut self, attribute: &Attribute, data: &[u8]) -> Control {
        self.handler.basic_offset_table(&attribute, data)
    }
    fn pixel_data_fragment(&mut self, attribute: &Attribute, data: &[u8]) -> Control {
        self.handler.pixel_data_fragment(&attribute, data)
    }
}
