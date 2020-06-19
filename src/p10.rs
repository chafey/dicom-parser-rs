use crate::data_set_parser::parse_full;
use crate::encoding::ExplicitBigEndian;
use crate::encoding::ExplicitLittleEndian;
use crate::encoding::ImplicitLittleEndian;
use crate::handler::Handler;
use crate::meta_information;
use crate::meta_information::MetaInformation;
use crate::value_parser::ParseError;

/// Parses a DICOM P10 Instance.  Returns the corresponding MetaInformation or
/// a ParseError if an error occurs during parse.
///
/// # Arguments
///
/// * `handler` - The Handler to invoke when parsing the DataSet
/// * `bytes`   - bytes from a DICOM P10 instance.  Can be the entire file or
///               the beginning part of the file.  If the entire file is not
///               provided and the parse is not Cancelled, an error may be
///               returned
///
pub fn parse<'a, T: Handler>(
    handler: &'a mut T,
    bytes: &[u8],
) -> Result<MetaInformation, ParseError> {
    let meta = meta_information::parse(handler, &bytes)?;
    let remaining_bytes = &bytes[meta.end_position..];
    let result = match &meta.transfer_syntax_uid[..] {
        "1.2.840.10008.1.2" => {
            // implicit little endian
            parse_full::<ImplicitLittleEndian>(handler, remaining_bytes, meta.end_position)
        }
        "1.2.840.10008.1.2.2" => {
            // explicit big endian
            parse_full::<ExplicitBigEndian>(handler, remaining_bytes, meta.end_position)
        }
        "1.2.840.10008.1.2.1.99" => panic!("deflated not suported yet"),
        _ => {
            // explicit little endian
            parse_full::<ExplicitLittleEndian>(handler, remaining_bytes, meta.end_position)
        }
    };
    match result {
        Ok(_) => Ok(meta),
        Err(parse_error) => Err(parse_error),
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
        assert_eq!(handler.dataset.attributes.len(), 7);
    }
    #[test]
    fn explicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(265, handler.dataset.attributes.len());
    }

    #[test]
    fn implicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.implicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(265, handler.dataset.attributes.len());
    }

    #[test]
    fn explicit_big_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.explicit_big_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(265, handler.dataset.attributes.len());
    }

    #[test]
    fn ele_sequences_known_lengths() {
        //(0008,9121) @ position 0x376 / 886
        let mut bytes = read_file("tests/fixtures/CT0012.fragmented_no_bot_jpeg_ls.80.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(165, handler.dataset.attributes.len());
    }

    #[test]
    fn ile_sequences_undefined_lengths() {
        //(0008,9121) @ position 0x376 / 886
        let mut bytes = read_file("tests/fixtures/IM00001.implicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let result = parse(&mut handler, &mut bytes);
        assert!(result.is_ok());
        assert_eq!(102, handler.dataset.attributes.len());
    }
}
