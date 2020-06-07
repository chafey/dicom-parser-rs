use crate::attribute::Attribute;
use crate::parser::{Callback, Control};

type ConditionFN = fn(&Attribute)->bool; 

pub struct Accumulator {
    pub filter: ConditionFN,
    pub stop: ConditionFN,
    pub attributes: Vec<Attribute>,
    pub data: Vec<Vec<u8>>
}


impl Accumulator {
    pub fn new(filter: ConditionFN, stop: ConditionFN,) -> Accumulator {
        Accumulator {
            filter,
            stop,
            attributes: vec![],
            data: vec![]
        }
    }
}

impl Callback for Accumulator {
    fn element(&mut self, attribute: Attribute) -> Control {
        println!("{:?}", attribute);
        if (self.filter)(&attribute) {
            return Control::Element;
        }
        if (self.stop)(&attribute) {
            return Control::Stop;
        }
        self.attributes.push(attribute);
        Control::Data
    }

    fn data(&mut self, data: &[u8]) {
        //println!("data of len {:?}", data.len());
        self.data.push(data.to_vec());
    }
}
