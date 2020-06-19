use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;
use crate::value_parser::ValueParser;
use std::marker::PhantomData;

#[derive(Default)]
pub struct DataUndefinedLengthParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + Encoding> ValueParser<T> for DataUndefinedLengthParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        _position: usize,
    ) -> Result<ParseResult, ParseError> {
        // we return immediately if we don't have at least 8 bytes of data
        // since it takes 8 bytes for the end of data marker
        if bytes.len() < 8 {
            return Ok(ParseResult::incomplete(0));
        }

        // scan for end of marker
        let (length, end_of_data_marker_found) = find_end_of_data_marker::<T>(bytes);

        // notify handler of data
        handler.data(attribute, &bytes[..length], end_of_data_marker_found);

        if end_of_data_marker_found {
            // include the size of the end of data marker in the number of
            // bytes consumed
            Ok(ParseResult::completed(length + 8))
        } else {
            Ok(ParseResult::incomplete(length))
        }
    }
}

fn find_end_of_data_marker<T: Encoding>(bytes: &[u8]) -> (usize, bool) {
    let mut position = 0;
    while position <= bytes.len() - 4 {
        let group = T::u16(&bytes[position..position + 2]);
        position += 2;
        if group == 0xFFFE {
            let element = T::u16(&bytes[position..position + 2]);
            if element == 0xE0DD {
                return (position - 2, true);
            }
        }
    }

    // we get here if we don't find the end of data.  We don't want
    // to send any bytes from the end of data marker and since it
    // is possible that bytes currently has some of the end of data
    // marker already, we reduce the numnber of bytes to send to
    // the handler by the size of the end of data marker (8).
    (bytes.len() - 8, false)
}

#[cfg(test)]
mod tests {

    use super::DataUndefinedLengthParser;
    use crate::attribute::Attribute;
    use crate::encoding::ExplicitLittleEndian;
    use crate::handler::data_set::DataSetHandler;
    use crate::value_parser::ParseState;
    use crate::value_parser::ValueParser;

    fn make_undefined_length_value() -> Vec<u8> {
        let mut bytes = vec![0; 250];
        // end with sequence item delimeter
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0xDD, 0xE0, 0, 0, 0, 0]);

        bytes
    }

    #[test]
    fn full_parse_completes() {
        let mut parser = DataUndefinedLengthParser::<ExplicitLittleEndian>::default();
        let mut handler = DataSetHandler::default();
        let mut attribute = Attribute::default();
        attribute.length = 0xFFFF_FFFF;
        let bytes = make_undefined_length_value();
        match parser.parse(&mut handler, &attribute, &bytes, 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, bytes.len());
                assert_eq!(result.state, ParseState::Completed);
            }
            Err(_error) => {
                assert!(false); // should not happen
            }
        };
    }

    #[test]
    fn dul_streaming_parse_completes() {
        let mut parser = DataUndefinedLengthParser::<ExplicitLittleEndian>::default();
        let mut handler = DataSetHandler::default();
        let mut attribute = Attribute::default();
        attribute.length = 0xFFFF_FFFF;
        let bytes = make_undefined_length_value();
        match parser.parse(&mut handler, &attribute, &bytes[0..100], 0) {
            Ok(result1) => {
                assert_eq!(result1.bytes_consumed, 92);
                assert_eq!(result1.state, ParseState::Incomplete);
                match parser.parse(
                    &mut handler,
                    &attribute,
                    &bytes[result1.bytes_consumed..],
                    0,
                ) {
                    Ok(result2) => {
                        assert_eq!(result2.bytes_consumed, bytes.len() - result1.bytes_consumed);
                        assert_eq!(result2.state, ParseState::Completed);
                    }
                    Err(_error) => {
                        assert!(false); // should not happen
                    }
                };
            }
            Err(_error) => {
                assert!(false); // should not happen
            }
        };
    }
}
