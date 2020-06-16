use crate::encoding::ExplicitBigEndian;
use crate::encoding::ExplicitLittleEndian;
use crate::encoding::ImplicitLittleEndian;
use crate::handler::Handler;
use crate::meta_information;
use crate::meta_information::MetaInformation;
use crate::parser::data_set::parse_full;
use crate::parser::ParseError;

pub fn parse<'a, T: Handler>(
    callback: &'a mut T,
    bytes: &mut [u8],
) -> Result<MetaInformation, ParseError> {
    let meta = meta_information::parse(&bytes).unwrap();
    let remaining_bytes = &bytes[meta.end_position..];
    let result = match &meta.transfer_syntax_uid[..] {
        "1.2.840.10008.1.2" => {
            // implicit little endian
            parse_full::<ImplicitLittleEndian>(callback, remaining_bytes)
        }
        "1.2.840.10008.1.2.2" => {
            // explicit big endian
            parse_full::<ExplicitBigEndian>(callback, remaining_bytes)
        }
        "1.2.840.10008.1.2.1.99" => panic!("deflated not suported yet"),
        _ => {
            // explicit little endian
            parse_full::<ExplicitLittleEndian>(callback, remaining_bytes)
        }
    };
    match result {
        Ok(_) => Ok(meta),
        Err(_bytes_remaining) => Err(ParseError {}),
    }
}

#[cfg(test)]
mod tests {

    use super::parse;
    use crate::handler::data_set::DataSetHandler;
    use crate::meta_information::tests::make_p10_header;
    use crate::test::tests::read_file;

    fn make_p10_file() -> Vec<u8> {
        let mut bytes = make_p10_header();
        bytes.extend_from_slice(&vec![0x08, 0x00, 0x05, 0x00, b'C', b'S', 2, 0, b'I', b'S']);

        bytes
    }

    #[test]
    fn explicit_little_endian_parses() {
        let mut bytes = make_p10_file();
        let mut handler = DataSetHandler::default();
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(handler.dataset.attributes.len(), 1);
    }
    #[test]
    fn explicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(257, handler.dataset.attributes.len());
    }

    #[test]
    fn implicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.implicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(257, handler.dataset.attributes.len());
    }

    #[test]
    fn explicit_big_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.explicit_big_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(257, handler.dataset.attributes.len());
    }

    #[test]
    fn ele_sequences_known_lengths() {
        //(0008,9121) @ position 0x376 / 886
        let mut bytes = read_file("tests/fixtures/CT0012.fragmented_no_bot_jpeg_ls.80.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(157, handler.dataset.attributes.len());
    }

    #[test]
    fn ile_sequences_undefined_lengths() {
        //(0008,9121) @ position 0x376 / 886
        let mut bytes = read_file("tests/fixtures/IM00001.implicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(94, handler.dataset.attributes.len());
    }
}
