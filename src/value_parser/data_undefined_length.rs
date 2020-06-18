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
        // scan for sequence delimitation item
        let (data_length, complete) = match find_end_of_data::<T>(bytes) {
            Err(()) => (bytes.len() - 8, false),
            Ok(data_length) => (data_length, true),
        };

        // notify handler of data
        handler.data(attribute, &bytes[..data_length]);

        if complete {
            Ok(ParseResult::completed(data_length + 8))
        } else {
            Ok(ParseResult::incomplete(data_length))
        }
    }
}

fn find_end_of_data<T: Encoding>(bytes: &[u8]) -> Result<usize, ()> {
    let mut position = 0;
    while position <= bytes.len() - 4 {
        let group = T::u16(&bytes[position..position + 2]);
        position += 2;
        if group == 0xFFFE {
            let element = T::u16(&bytes[position..position + 2]);
            if element == 0xE0DD {
                // TODO: Consider verifying zero length?

                return Ok(position - 2);
            }
        }
    }

    Err(())
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
