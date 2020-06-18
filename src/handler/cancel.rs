use crate::attribute::Attribute;
use crate::handler::{Handler, HandlerResult};

pub type CancelFN = fn(&Attribute) -> bool;

/// Implements the Handler trait that will cancel the parse if the provided
/// Cancel function returns true or forward/proxy all functions to another
/// Handler implementation.  Some use cases do not require parsing the full
/// DICOM DataSet and encapsulating the stop parsing in a function (perhaps
/// a closure) can aid in readability of processing logic.
pub struct CancelHandler<'t> {
    /// true if the parse has been canceled, false otherwise
    pub canceled: bool,
    /// the Handler to forward/proxy function calls to
    pub handler: &'t mut dyn Handler,
    /// The function to call to see if the parse should be cancelled
    pub cancel_fn: CancelFN,
}

impl<'t> CancelHandler<'t> {
    /// Creates a new CancelHanlder given a handler to proxy/forward functions
    /// to and a function that returns true when the parse should be canceled
    pub fn new(handler: &'t mut dyn Handler, cancel_fn: CancelFN) -> CancelHandler<'t> {
        CancelHandler {
            canceled: false,
            handler,
            cancel_fn,
        }
    }
}

impl Handler for CancelHandler<'_> {
    fn attribute(
        &mut self,
        attribute: &Attribute,
        position: usize,
        data_offset: usize,
    ) -> HandlerResult {
        if (self.cancel_fn)(&attribute) {
            self.canceled = true;
            return HandlerResult::Cancel;
        }
        self.handler.attribute(&attribute, position, data_offset)
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
    fn basic_offset_table(&mut self, attribute: &Attribute, data: &[u8]) -> HandlerResult {
        self.handler.basic_offset_table(&attribute, data)
    }
    fn pixel_data_fragment(&mut self, attribute: &Attribute, data: &[u8]) -> HandlerResult {
        self.handler.pixel_data_fragment(&attribute, data)
    }
}
