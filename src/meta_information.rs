use crate::attribute::Attribute;
use crate::parser::Parser;
use crate::accumulator::Accumulator;
use crate::prefix;
use crate::condition;

pub struct MetaInformation {
    pub media_storage_sop_class_uid: String,
    pub media_storage_sop_instance_uid: String,
    pub transfer_syntax_uid: String,
    pub implementation_class_uid: String
}

pub fn parse(bytes: &[u8]) -> Result<Vec<Attribute>, ()> {
    
    if prefix::detect(bytes) == false {
        return Err(());
    }

    let stop_if_not_group_2 = |x:&Attribute| {
        x.tag.group != 2
    };

    let accumulator = Accumulator::new(condition::none, stop_if_not_group_2);
    let mut parser = Parser::<Accumulator>::new(accumulator);
    parser.parse(&bytes[132..]);

    let result = parser.callback.attributes;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::parse;
    
    #[test]
    fn valid_meta_information() {
        let mut bytes = vec![];
        bytes.resize(144, 0);
        bytes[128] = 'D' as u8;
        bytes[129] = 'I' as u8;
        bytes[130] = 'C' as u8;
        bytes[131] = 'M' as u8;
        bytes[132] = 2;
        bytes[133] = 0;
        bytes[134] = 1;
        bytes[135] = 0;
        bytes[136] = 0x4F;
        bytes[137] = 0x42;
        bytes[138] = 0;
        bytes[139] = 0;
        bytes[140] = 0;
        bytes[141] = 0;
        bytes[142] = 0;
        bytes[143] = 0;
        
        let attrs = parse(&bytes).unwrap();
        println!("{:?}", attrs);
    }
}