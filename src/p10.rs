use crate::attribute::Attribute;
use crate::attribute::AttributeFN;
use crate::dataset::{Callback, Parser};
use crate::meta_information;
use crate::meta_information::MetaInformation;

fn get_attribute_fn(transfer_syntax_uid: &str) -> AttributeFN {
    match transfer_syntax_uid {
        "1.2.840.10008.1.2" => Attribute::ile,
        _ => Attribute::ele,
    }
}

pub fn parse<'a, T: Callback>(
    callback: &'a mut T,
    bytes: &mut [u8],
) -> Result<MetaInformation, ()> {
    let meta = meta_information::parse(&bytes).unwrap();
    let attribute_fn = get_attribute_fn(&meta.transfer_syntax_uid[..]);
    let mut parser = Parser::new(callback, attribute_fn);
    parser.parse(&bytes[meta.end_position..]);
    Ok(meta)
}

#[cfg(test)]
mod tests {

    use super::parse;
    use crate::accumulator::Accumulator;
    use crate::condition;
    use crate::meta_information::tests::make_p10_header;

    fn make_p10_file() -> Vec<u8> {
        let mut bytes = make_p10_header();
        bytes.extend_from_slice(&vec![0x08, 0x00, 0x05, 0x00, b'C', b'S', 2, 0, b'I', b'S']);

        bytes
    }

    #[test]
    fn explicit_little_endian_parses() {
        let mut bytes = make_p10_file();
        let mut accumulator = Accumulator::new(condition::none, condition::none);
        parse(&mut accumulator, &mut bytes).unwrap();
        assert_eq!(accumulator.attributes.len(), 1);
    }
}
