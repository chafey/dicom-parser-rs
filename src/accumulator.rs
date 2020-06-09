use crate::attribute::Attribute;
use crate::parser::handler::{Control, Handler};
use crate::vr::VR;

type ConditionFN = fn(&Attribute) -> bool;

pub struct Accumulator {
    pub filter: ConditionFN,
    pub stop: ConditionFN,
    pub attributes: Vec<Attribute>,
    pub data: Vec<Vec<u8>>,
    pub depth: usize,
    pub print: bool,
}

impl Accumulator {
    pub fn new(filter: ConditionFN, stop: ConditionFN) -> Accumulator {
        Accumulator {
            filter,
            stop,
            attributes: vec![],
            data: vec![],
            depth: 0,
            print: false,
        }
    }
}

impl Handler for Accumulator {
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
        self.attributes.push(*attribute);
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
        self.data.push(data.to_vec());
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
}
