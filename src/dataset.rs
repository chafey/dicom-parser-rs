use crate::attribute::Attribute;

#[derive(Debug)]
pub struct DataSet {
    pub attributes: Vec<Attribute>,
    pub data: Vec<Vec<u8>>,
}

impl DataSet {
    pub fn new() -> DataSet {
        DataSet {
            attributes: vec![],
            data: vec![]
        }
    }
}