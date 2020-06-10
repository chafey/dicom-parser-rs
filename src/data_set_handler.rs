use crate::attribute::Attribute;
use crate::condition::ConditionFN;
use crate::data_set::DataSet;
use crate::parser::handler::{Control, Handler};
use crate::vr::VR;

pub struct DataSetHandler {
    pub filter: ConditionFN,
    pub stop: ConditionFN,
    pub dataset: DataSet,
    pub depth: usize,
    pub print: bool,
}

impl DataSetHandler {
    pub fn new(filter: ConditionFN, stop: ConditionFN) -> DataSetHandler {
        DataSetHandler {
            filter,
            stop,
            dataset: DataSet::default(),
            depth: 0,
            print: false,
        }
    }
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
        if (self.filter)(&attribute) {
            return Control::Element;
        }
        if (self.stop)(&attribute) {
            return Control::Stop;
        }
        self.dataset.attributes.push(*attribute);
        Control::Data
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
        Control::Data
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
        Control::Data
    }
}
