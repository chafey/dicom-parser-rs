use crate::attribute::Attribute;
use crate::data_set::DataSet;
use crate::parser::handler::{Control, Handler};
use crate::vr::VR;

#[derive(Default)]
pub struct DataSetHandler {
    pub dataset: DataSet,
    pub depth: usize,
    pub print: bool,
}

impl Handler for DataSetHandler {
    fn element(&mut self, attribute: &Attribute) -> Control {
        if self.print {
            println!("{: <width$}{:?}", "", attribute, width = (self.depth * 2));
            if let Some(vr) = attribute.vr {
                if vr == VR::SQ {
                    println!("{: <width$}\\/", "", width = (self.depth * 2));
                }
            }
        }
        self.dataset.attributes.push(*attribute);
        Control::Continue
    }

    fn data(&mut self, _attribute: &Attribute, data: &[u8]) {
        if self.print {
            println!(
                "{: <width$}  \\ data of len {:?}",
                " ",
                data.len(),
                width = (self.depth * 2)
            );
        }
        self.dataset.data.push(data.to_vec());
    }

    fn start_sequence_item(&mut self, _attribute: &Attribute) {
        if self.print {
            println!("{: <width$} {{", "", width = (self.depth * 2));
        }
        self.depth += 1;
    }

    fn end_sequence_item(&mut self, _attribute: &Attribute) {
        self.depth -= 1;
        if self.print {
            println!("{: <width$} }}", "", width = (self.depth * 2));
        }
    }

    fn basic_offset_table(&mut self, _attribute: &Attribute, data: &[u8]) -> Control {
        if self.print {
            println!(
                "{: <width$}  \\ basic offsett table of len {:?}",
                " ",
                data.len(),
                width = (self.depth * 2)
            );
        }
        //self.dataset.data.push(data.to_vec());
        Control::Continue
    }

    fn pixel_data_fragment(&mut self, _attribute: &Attribute, data: &[u8]) -> Control {
        if self.print {
            println!(
                "{: <width$}  \\ pixel data fragment of len {:?}",
                " ",
                data.len(),
                width = (self.depth * 2)
            );
        }
        self.dataset.data.push(data.to_vec());
        Control::Continue
    }
}
