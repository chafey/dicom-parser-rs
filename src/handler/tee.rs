use crate::attribute::Attribute;
use crate::handler::{Handler, HandlerResult};

/// Implements the Handler trait that forward each function call to each
/// handler in its list of handlers
#[derive(Default)]
pub struct TeeHandler<'t> {
    /// the Handlers to forward function calls to
    pub handlers: Vec<&'t mut dyn Handler>,
}

impl Handler for TeeHandler<'_> {
    fn attribute(
        &mut self,
        attribute: &Attribute,
        position: usize,
        data_offset: usize,
    ) -> HandlerResult {
        for handler in &mut self.handlers {
            handler.attribute(attribute, position, data_offset);
        }
        HandlerResult::Continue
    }
    fn data(&mut self, attribute: &Attribute, data: &[u8]) {
        for handler in &mut self.handlers {
            handler.data(attribute, data);
        }
    }
    fn start_sequence(&mut self, attribute: &Attribute) {
        for handler in &mut self.handlers {
            handler.start_sequence(attribute);
        }
    }
    fn start_sequence_item(&mut self, attribute: &Attribute) {
        for handler in &mut self.handlers {
            handler.start_sequence_item(attribute);
        }
    }
    fn end_sequence_item(&mut self, attribute: &Attribute) {
        for handler in &mut self.handlers {
            handler.end_sequence_item(attribute);
        }
    }
    fn end_sequence(&mut self, attribute: &Attribute) {
        for handler in &mut self.handlers {
            handler.end_sequence(attribute);
        }
    }
    fn basic_offset_table(&mut self, attribute: &Attribute, data: &[u8]) -> HandlerResult {
        for handler in &mut self.handlers {
            handler.basic_offset_table(attribute, data);
        }

        HandlerResult::Continue
    }
    fn pixel_data_fragment(
        &mut self,
        attribute: &Attribute,
        fragment_number: usize,
        data: &[u8],
    ) -> HandlerResult {
        for handler in &mut self.handlers {
            handler.pixel_data_fragment(attribute, fragment_number, data);
        }
        HandlerResult::Continue
    }
}
