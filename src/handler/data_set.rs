use crate::attribute::Attribute;
use crate::data_set::DataSet;
use crate::handler::{Handler, HandlerResult};

#[derive(Default)]
pub struct DataSetHandler {
    pub dataset: DataSet,
    pub depth: usize,
    pub print: bool,
}

/// Implementation of Handler that prints out the result and collects all
/// attributes and associated data in a DataSet instance
impl Handler for DataSetHandler {
    fn attribute(
        &mut self,
        attribute: &Attribute,
        position: usize,
        data_offset: usize,
    ) -> HandlerResult {
        if self.print {
            println!(
                "{:-<width$}{:?} (position={}, data_offset={})",
                "-",
                attribute,
                position,
                data_offset,
                width = (self.depth * 2)
            );
        }
        self.dataset.attributes.push(*attribute);
        HandlerResult::Continue
    }

    fn data(&mut self, _attribute: &Attribute, data: &[u8], complete: bool) {
        if self.print {
            println!(
                "{:-<width$}+ data of len {:?} complete={}",
                "-",
                data.len(),
                complete,
                width = (self.depth * 2)
            );
        }
        self.dataset.data.push(data.to_vec());
    }

    fn start_sequence(&mut self, _attribute: &Attribute) {
        if self.print {
            println!("{:-<width$}[", "-", width = (self.depth * 2));
        }
        self.depth += 1;
    }

    fn start_sequence_item(&mut self, _attribute: &Attribute) {
        if self.print {
            println!("{:-<width$}{{", "-", width = (self.depth * 2));
        }
        self.depth += 1;
    }

    fn end_sequence_item(&mut self, _attribute: &Attribute) {
        self.depth -= 1;
        if self.print {
            println!("{:-<width$}}}", "-", width = (self.depth * 2));
        }
    }

    fn end_sequence(&mut self, _attribute: &Attribute) {
        self.depth -= 1;
        if self.print {
            println!("{: <width$}]", "-", width = (self.depth * 2));
        }
    }

    fn basic_offset_table(
        &mut self,
        _attribute: &Attribute,
        data: &[u8],
        complete: bool,
    ) -> HandlerResult {
        if self.print {
            println!(
                "{:-<width$}  \\ basic offset table of len {:?} complete={}",
                "-",
                data.len(),
                complete,
                width = (self.depth * 2)
            );
        }
        HandlerResult::Continue
    }

    fn pixel_data_fragment(
        &mut self,
        _attribute: &Attribute,
        fragment_number: usize,
        data: &[u8],
        complete: bool,
    ) -> HandlerResult {
        if self.print {
            println!(
                "{:-<width$}  \\ pixel data fragment #{} data of len {:?} complete={}",
                "-",
                fragment_number,
                data.len(),
                complete,
                width = (self.depth * 2)
            );
        }
        self.dataset.data.push(data.to_vec());
        HandlerResult::Continue
    }
}
