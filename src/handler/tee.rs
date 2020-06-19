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
        if self
            .handlers
            .iter_mut()
            .map(|handler| handler.attribute(attribute, position, data_offset))
            .filter(|handler_result| handler_result == &HandlerResult::Cancel)
            .count()
            > 0
        {
            HandlerResult::Cancel
        } else {
            HandlerResult::Continue
        }
    }
    fn data(&mut self, attribute: &Attribute, data: &[u8], complete: bool) {
        self.handlers
            .iter_mut()
            .for_each(|handler| handler.data(attribute, data, complete))
    }
    fn start_sequence(&mut self, attribute: &Attribute) {
        self.handlers
            .iter_mut()
            .for_each(|handler| handler.start_sequence(attribute))
    }
    fn start_sequence_item(&mut self, attribute: &Attribute) {
        self.handlers
            .iter_mut()
            .for_each(|handler| handler.start_sequence_item(attribute))
    }
    fn end_sequence_item(&mut self, attribute: &Attribute) {
        self.handlers
            .iter_mut()
            .for_each(|handler| handler.end_sequence_item(attribute))
    }
    fn end_sequence(&mut self, attribute: &Attribute) {
        self.handlers
            .iter_mut()
            .for_each(|handler| handler.end_sequence(attribute))
    }
    fn basic_offset_table(&mut self, attribute: &Attribute, data: &[u8], complete: bool) -> HandlerResult {
        if self
            .handlers
            .iter_mut()
            .map(|handler| handler.basic_offset_table(attribute, data, complete))
            .filter(|handler_result| handler_result == &HandlerResult::Cancel)
            .count()
            > 0
        {
            HandlerResult::Cancel
        } else {
            HandlerResult::Continue
        }
    }
    fn pixel_data_fragment(
        &mut self,
        attribute: &Attribute,
        fragment_number: usize,
        data: &[u8],
        complete: bool
    ) -> HandlerResult {
        if self
            .handlers
            .iter_mut()
            .map(|handler| handler.pixel_data_fragment(attribute, fragment_number, data, complete))
            .filter(|handler_result| handler_result == &HandlerResult::Cancel)
            .count()
            > 0
        {
            HandlerResult::Cancel
        } else {
            HandlerResult::Continue
        }
    }
}
