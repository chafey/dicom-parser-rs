use crate::attribute::Attribute;

#[derive(Default, Debug)]
pub struct DataSet {
    pub attributes: Vec<Attribute>,
    pub data: Vec<Vec<u8>>,
}

