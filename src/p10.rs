use crate::attribute::Attribute;
use crate::attribute::AttributeFN;
use crate::dataset::{Callback, Parser};
use crate::meta_information;
use crate::meta_information::MetaInformation;

fn get_attribute_fn(meta: &MetaInformation) -> AttributeFN {
    match &meta.transfer_syntax_uid[..] {
        "1.2.840.10008.1.2" => Attribute::ile,
        _ => Attribute::ele,
    }
}

pub fn parse<'a, T: Callback>(
    callback: &'a mut T,
    bytes: &mut [u8],
) -> Result<MetaInformation, ()> {
    let meta = meta_information::parse(&bytes).unwrap();
    let attribute_fn = get_attribute_fn(&meta);
    //println!("meta: {:?}", meta);
    let mut parser = Parser::new(callback, attribute_fn);
    parser.parse(&bytes[meta.end_position..]);
    Ok(meta)
}
/*

#[cfg(test)]
mod tests {

    use super::parse;
    use crate::accumulator::Accumulator;
    use crate::condition;

    #[test]
    fn explicit_little_endian_2() {
        let mut bytes = vec![];
        let mut accumulator = Accumulator::new(condition::none, condition::none);
        parse(&mut accumulator, &mut bytes).unwrap();
        println!("Parsed {:?} attributes", accumulator.attributes.len());
        println!("{:?}", accumulator.attributes);
    }
}*/
