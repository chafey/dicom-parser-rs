use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::cancel::CancelHandler;
use crate::handler::Handler;
use crate::parser::data_set::DataSetParser;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag;

pub struct SequenceItemDataParser<T: Encoding> {
    item_length: usize,
    parser: DataSetParser<T>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> SequenceItemDataParser<T> {
    pub fn new(item_length: usize) -> SequenceItemDataParser<T> {
        SequenceItemDataParser {
            item_length,
            parser: DataSetParser::<T>::default(),
            total_bytes_consumed: 0,
        }
    }
}

impl<T: 'static + Encoding> Parser<T> for SequenceItemDataParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        // if we have a known length, only parse the bytes we know we have
        let remaining_bytes = if self.item_length == 0xFFFF_FFFF
            || bytes.len() < (self.item_length - self.total_bytes_consumed)
        {
            bytes
        } else {
            &bytes[0..(self.item_length - self.total_bytes_consumed)]
        };

        let mut sequence_item_handler =
            CancelHandler::new(handler, |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM);

        let parse_result = self
            .parser
            .parse(&mut sequence_item_handler, remaining_bytes)?;

        self.total_bytes_consumed += parse_result.bytes_consumed;

        if sequence_item_handler.canceled {
            self.total_bytes_consumed += 8;
        }

        if sequence_item_handler.canceled {
            Ok(ParseResult::completed(parse_result.bytes_consumed + 8))
        } else if self.total_bytes_consumed == self.item_length {
            Ok(ParseResult::completed(parse_result.bytes_consumed))
        } else if self.total_bytes_consumed < self.item_length {
            Ok(ParseResult::incomplete(parse_result.bytes_consumed))
        } else {
            Ok(parse_result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceItemDataParser;
    use crate::data_set::DataSet;
    use crate::encoding::ExplicitLittleEndian;
    use crate::handler::data_set::DataSetHandler;
    use crate::parser::ParseState;
    use crate::parser::Parser;

    fn make_sequence_item_undefined_length() -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x00, 0x00, b'U', b'L', 4, 0, 0, 0, 0, 0]);
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0x0D, 0xE0, 0, 0, 0, 0]);

        bytes
    }

    fn make_sequence_item_known_length() -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x00, 0x00, b'U', b'L', 4, 0, 0, 0, 0, 0]);

        bytes
    }

    #[test]
    fn undefined_length_completes() {
        let bytes = make_sequence_item_undefined_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(0xFFFF_FFFF);
        let mut handler = DataSetHandler {
            dataset: DataSet::default(),
            depth: 0,
            print: false,
        };
        let result = parser.parse(&mut handler, &bytes[..]).unwrap();
        assert_eq!(result.bytes_consumed, 20);
        assert_eq!(result.state, ParseState::Completed);
    }

    #[test]
    fn partial_undefined_length_returns_incomplete() {
        let bytes = make_sequence_item_undefined_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(0xFFFF_FFFF);
        let mut handler = DataSetHandler {
            dataset: DataSet::default(),
            depth: 0,
            print: false,
        };
        let result = parser.parse(&mut handler, &bytes[0..1]).unwrap();
        assert_eq!(result.bytes_consumed, 0);
        assert_eq!(result.state, ParseState::Incomplete);
    }

    #[test]
    fn partial_in_item_delimetation_item_undefined_length_returns_incomplete() {
        let bytes = make_sequence_item_undefined_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(0xFFFF_FFFF);
        let mut handler = DataSetHandler {
            dataset: DataSet::default(),
            depth: 0,
            print: false,
        };
        let result = parser.parse(&mut handler, &bytes[0..13]).unwrap();
        assert_eq!(result.bytes_consumed, 12);
        assert_eq!(result.state, ParseState::Incomplete);
    }

    #[test]
    fn known_length_completes() {
        let bytes = make_sequence_item_known_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(bytes.len());
        let mut handler = DataSetHandler {
            dataset: DataSet::default(),
            depth: 0,
            print: false,
        };
        let result = parser.parse(&mut handler, &bytes[..]).unwrap();
        assert_eq!(result.bytes_consumed, 12);
        assert_eq!(result.state, ParseState::Completed);
    }

    #[test]
    fn partial_known_length_returns_incomplete() {
        let bytes = make_sequence_item_known_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(bytes.len());
        let mut handler = DataSetHandler {
            dataset: DataSet::default(),
            depth: 0,
            print: false,
        };
        let result = parser.parse(&mut handler, &bytes[..1]).unwrap();
        assert_eq!(result.bytes_consumed, 0);
        assert_eq!(result.state, ParseState::Incomplete);
    }
}
