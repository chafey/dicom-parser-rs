use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;
use crate::value_parser::ValueParser;
use std::marker::PhantomData;

/// Parses the value field for an Attribute.  
#[derive(Default)]
pub struct DataParser<T: Encoding> {
    pub phantom: PhantomData<T>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> ValueParser<T> for DataParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        _position: usize,
    ) -> Result<ParseResult, ParseError> {
        // get slice of bytes based on how many we have already parsed for this value field
        let bytes_remaining = if bytes.len() > attribute.length - self.total_bytes_consumed {
            &bytes[attribute.length - self.total_bytes_consumed..]
        } else {
            bytes
        };

        // notify handler of data
        let complete = self.total_bytes_consumed + bytes_remaining.len() == attribute.length;
        handler.data(attribute, bytes_remaining, complete);

        // update our internal counter of bytes consumed
        self.total_bytes_consumed += bytes_remaining.len();

        // Return complete if we have parsed all bytes for this value or incomplete if not
        if self.total_bytes_consumed == attribute.length {
            Ok(ParseResult::completed(bytes_remaining.len()))
        } else {
            Ok(ParseResult::incomplete(bytes_remaining.len()))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::DataParser;
    use crate::attribute::Attribute;
    use crate::encoding::ExplicitLittleEndian;
    use crate::handler::data_set::DataSetHandler;
    use crate::value_parser::ParseState;
    use crate::value_parser::ValueParser;

    #[test]
    fn data_parser_completes() {
        let mut parser = DataParser::<ExplicitLittleEndian>::default();
        let mut handler = DataSetHandler::default();
        let mut attribute = Attribute::default();
        attribute.length = 255;
        let bytes = [0; 255];
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
    fn data_parser_streaming_completes() {
        let mut parser = DataParser::<ExplicitLittleEndian>::default();
        let mut handler = DataSetHandler::default();
        let mut attribute = Attribute::default();
        attribute.length = 255;
        let bytes = [0; 255];
        match parser.parse(&mut handler, &attribute, &bytes[0..100], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 100);
                assert_eq!(result.state, ParseState::Incomplete);
            }
            Err(_error) => {
                assert!(false); // should not happen
            }
        };
        match parser.parse(&mut handler, &attribute, &bytes[100..], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, bytes.len() - 100);
                assert_eq!(result.state, ParseState::Completed);
            }
            Err(_error) => {
                assert!(false); // should not happen
            }
        };
    }
}
