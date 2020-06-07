use crate::attribute::Attribute;
use crate::parser::{Callback, Control};

pub struct Accumulator {
    pub attributes: Vec<Attribute>,
    pub data: Vec<Vec<u8>>
}

impl Accumulator {
    pub fn new() -> Accumulator {
        Accumulator {
            attributes: vec![],
            data: vec![]
        }
    }
}

impl Callback for Accumulator {
    fn element(&mut self, attribute: Attribute) -> Control {
        println!("{:?}", attribute);
        self.attributes.push(attribute);
        Control::Element
    }

    fn data(&mut self, data: &[u8]) {
        //println!("data of len {:?}", data.len());
        self.data.push(data.to_vec());
    }
}
