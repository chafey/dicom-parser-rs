use crate::attribute::Attribute;
use crate::handler::{Control, Handler};

pub type StopFN = fn(&Attribute) -> bool;

pub struct StopHandler<'t> {
    pub stopped: bool,
    pub handler: &'t mut dyn Handler,
    pub stop_fn: StopFN,
}

impl<'t> StopHandler<'t> {
    pub fn new(handler: &'t mut dyn Handler, stop_fn: StopFN) -> StopHandler<'t> {
        StopHandler {
            stopped: false,
            handler,
            stop_fn,
        }
    }
}

impl Handler for StopHandler<'_> {
    fn element(&mut self, attribute: &Attribute) -> Control {
        if (self.stop_fn)(&attribute) {
            self.stopped = true;
            return Control::Stop;
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
