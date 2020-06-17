use crate::attribute_parser::AttributeParser;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::ParseError;
use crate::parser::ParseResult;
use crate::parser::ParseState;

pub struct DataSetParser<T: Encoding> {
    parser: AttributeParser<T>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> DataSetParser<T> {
    pub fn default() -> DataSetParser<T> {
        DataSetParser {
            parser: AttributeParser::<T>::default(),
            total_bytes_consumed: 0,
        }
    }
}

impl<T: 'static + Encoding> DataSetParser<T> {
    pub fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
        bytes_from_beginning: usize,
    ) -> Result<ParseResult, ParseError> {
        let mut remaining_bytes = bytes;
        let mut bytes_consumed = 0;
        while !remaining_bytes.is_empty() {
            let position = bytes_from_beginning + bytes_consumed;
            let result = self.parser.parse(handler, remaining_bytes, position)?;
            bytes_consumed += result.bytes_consumed;
            self.total_bytes_consumed += result.bytes_consumed;
            remaining_bytes = &remaining_bytes[result.bytes_consumed..];
            match result.state {
                ParseState::Cancelled => {
                    return Ok(ParseResult::cancelled(bytes_consumed));
                }
                ParseState::Incomplete => {
                    return Ok(ParseResult::incomplete(bytes_consumed));
                }
                ParseState::Completed => {
                    self.parser = AttributeParser::<T>::default();
                    continue;
                }
            }
        }
        Ok(ParseResult::completed(bytes_consumed))
    }
}

// returns
//  number of bytes parsed
//  true if cancelled, false, if not
pub fn parse_full<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    bytes: &[u8],
    bytes_from_beginning: usize,
) -> Result<(usize, bool), ParseError> {
    let mut parser = DataSetParser::<T>::default();
    match parser.parse(handler, bytes, bytes_from_beginning) {
        Ok(parse_result) => match parse_result.state {
            ParseState::Cancelled => Ok((parse_result.bytes_consumed, true)),
            ParseState::Incomplete => Err(ParseError {
                reason: "unexpected EOF",
                position: parse_result.bytes_consumed + bytes_from_beginning,
            }),
            ParseState::Completed => Ok((parse_result.bytes_consumed, false)),
        },
        Err(parse_error) => Err(parse_error),
    }
}

#[cfg(test)]
mod tests {

    use super::DataSetParser;
    use crate::encoding::{ExplicitLittleEndian, ImplicitLittleEndian};
    use crate::handler::data_set::DataSetHandler;
    use crate::parser::ParseError;
    use crate::parser::ParseState;
    use crate::test::tests::read_data_set_bytes_from_file;

    fn parse_ele_data_set(
        bytes: &[u8],
        bytes_from_beginning: usize,
    ) -> Result<(ParseState, usize), ParseError> {
        let mut handler = DataSetHandler::default();
        let mut parser = DataSetParser::<ExplicitLittleEndian>::default();
        let result = parser.parse(&mut handler, bytes, bytes_from_beginning)?;
        Ok((result.state, result.bytes_consumed))
    }
    #[test]
    fn parse_full_ok() {
        let (meta, bytes) =
            read_data_set_bytes_from_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let result = parse_ele_data_set(&bytes[..], meta.end_position);
        assert!(result.is_ok());
    }
    /*

    fn split_parse(bytes: &[u8], split_position: usize) -> Result<(), ()> {
        println!("split_parse @ {} of {} ", split_position, bytes.len());
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let mut parser = DataSetParser::<ImplicitLittleEndian>::default();
        let result = parser.parse(&mut handler, &bytes[0..split_position])?;
        //println!("bytes_consumed: {:?}", result.bytes_consumed);
        //println!("state: {:?}", result.state);
        let result2 = parser.parse(&mut handler, &bytes[result.bytes_consumed..])?;
        //println!("bytes_consumed: {:?}", result2.bytes_consumed);
        //println!("state: {:?}", result2.state);
        assert_eq!(result2.bytes_consumed, bytes.len() - result.bytes_consumed);
        Ok(())
    }
    #[test]
    fn parse_partial_debug() {
        //let bytes = read_data_set_bytes_from_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        //let bytes = read_data_set_bytes_from_file("tests/fixtures/CT0012.fragmented_no_bot_jpeg_ls.80.dcm");
        let bytes =
            read_data_set_bytes_from_file("tests/fixtures/IM00001.implicit_little_endian.dcm"); // meta ends at 352

        // 3576 + 336 = 3912 xF48 (in VR of Pixel Data Attribute? )
        let result = split_parse(&bytes, 3576);
        assert!(result.is_ok());
        //println!("{:?}", result);
    }
    */

    #[test]
    fn explicit_little_endian_streaming_parse_ok() {
        let (meta, bytes) =
            read_data_set_bytes_from_file("tests/fixtures/CT0012.fragmented_no_bot_jpeg_ls.80.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let mut parser = DataSetParser::<ExplicitLittleEndian>::default();
        let mut offset = 0;
        for i in 0..bytes.len() {
            match parser.parse(&mut handler, &bytes[offset..i + 1], meta.end_position) {
                Ok(parse_result) => {
                    if parse_result.bytes_consumed != 0 {
                        //println!("consumed {} bytes", parse_result.bytes_consumed)
                    }
                    offset += parse_result.bytes_consumed;
                }
                Err(_error) => panic!("Parse Errored"),
            }
        }
        assert_eq!(157, handler.dataset.attributes.len());
    }

    #[test]
    fn implicit_little_endian_streaming_parse_ok() {
        let (meta, bytes) =
            read_data_set_bytes_from_file("tests/fixtures/IM00001.implicit_little_endian.dcm");
        let mut handler = DataSetHandler::default();
        //handler.print = true;
        let mut parser = DataSetParser::<ImplicitLittleEndian>::default();
        let mut offset = 0;
        for i in 0..bytes.len() {
            match parser.parse(&mut handler, &bytes[offset..i + 1], meta.end_position + i) {
                Ok(parse_result) => {
                    if parse_result.bytes_consumed != 0 {
                        //println!("consumed {} bytes", parse_result.bytes_consumed)
                    }
                    offset += parse_result.bytes_consumed;
                }
                Err(_error) => panic!("Parse Errored"),
            }
        }
        assert_eq!(94, handler.dataset.attributes.len());
    }
}
