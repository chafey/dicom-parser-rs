use crate::attribute::Attribute;
use crate::data_set_parser::DataSetParser;
use crate::encoding::Encoding;
use crate::handler::cancel::CancelHandler;
use crate::handler::Handler;
use crate::tag;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;
use crate::value_parser::ValueParser;

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

impl<T: 'static + Encoding> ValueParser<T> for SequenceItemDataParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        _attribute: &Attribute,
        bytes: &[u8],
        position: usize,
    ) -> Result<ParseResult, ParseError> {
        // if we have a known length, only parse the bytes we know we have
        let remaining_bytes = if self.item_length == 0xFFFF_FFFF
            || bytes.len() < (self.item_length - self.total_bytes_consumed)
        {
            bytes
        } else {
            &bytes[0..(self.item_length - self.total_bytes_consumed)]
        };

        // setup a cancel handler based on the item delimitation item
        let mut sequence_item_handler =
            CancelHandler::new(handler, |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM);

        // forward bytes to the DataSetParser
        let parse_result =
            self.parser
                .parse(&mut sequence_item_handler, remaining_bytes, position)?;

        // update internal state
        self.total_bytes_consumed += parse_result.bytes_consumed;

        // if the parse was cancelled due to the item delimitation item, add
        // the size of the item delimitation item to the bytes consumed
        if sequence_item_handler.canceled {
            self.total_bytes_consumed += 8;
        }

        if sequence_item_handler.canceled {
            // if the parse was cancelled due to hitting the item delimitation item,
            // we are complete
            Ok(ParseResult::completed(parse_result.bytes_consumed + 8))
        } else if self.total_bytes_consumed == self.item_length {
            // if we have a known length and have consumed all the bytes, we are complete
            Ok(ParseResult::completed(parse_result.bytes_consumed))
        } else if self.total_bytes_consumed < self.item_length {
            // if we have a known length and have consumed fewer bytes, we are incomplete
            Ok(ParseResult::incomplete(parse_result.bytes_consumed))
        } else {
            // not sure this can happen
            Ok(parse_result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceItemDataParser;
    use crate::attribute::Attribute;
    use crate::encoding::ExplicitLittleEndian;
    use crate::tag::Tag;
    use crate::test::tests::TestHandler;
    use crate::value_parser::ParseState;
    use crate::value_parser::ValueParser;

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
    fn known_length_completes() {
        let bytes = make_sequence_item_known_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(bytes.len());
        let mut handler = TestHandler::default();
        let attribute = Attribute {
            tag: Tag::new(0x0008, 0x0008),
            vr: None,
            length: 0,
        };
        match parser.parse(&mut handler, &attribute, &bytes[..], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 12);
                assert_eq!(result.state, ParseState::Completed);
            }
            Err(_parse_result) => panic!("Let's play Global Thermonuclear War"),
        }
    }

    #[test]
    fn partial_known_length_returns_incomplete() {
        let bytes = make_sequence_item_known_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(bytes.len());
        let mut handler = TestHandler::default();
        let attribute = Attribute {
            tag: Tag::new(0x0008, 0x0008),
            vr: None,
            length: 0,
        };
        match parser.parse(&mut handler, &attribute, &bytes[..1], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 0);
                assert_eq!(result.state, ParseState::Incomplete);
            }
            Err(_parse_error) => panic!("Let's play Global Thermonuclear War"),
        }
    }

    #[test]
    fn undefined_length_completes() {
        let bytes = make_sequence_item_undefined_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(0xFFFF_FFFF);
        let mut handler = TestHandler::default();
        let attribute = Attribute {
            tag: Tag::new(0x0008, 0x0008),
            vr: None,
            length: 0,
        };
        match parser.parse(&mut handler, &attribute, &bytes[..], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 20);
                assert_eq!(result.state, ParseState::Completed);
            }
            Err(_) => panic!("Let's play Global Thermonuclear War"),
        };
    }

    #[test]
    fn partial_undefined_length_returns_incomplete() {
        let bytes = make_sequence_item_undefined_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(0xFFFF_FFFF);
        let mut handler = TestHandler::default();
        let attribute = Attribute {
            tag: Tag::new(0x0008, 0x0008),
            vr: None,
            length: 0,
        };
        match parser.parse(&mut handler, &attribute, &bytes[0..1], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 0);
                assert_eq!(result.state, ParseState::Incomplete);
            }
            Err(_error) => {
                panic!("Let's play Global Thermonuclear War");
            }
        }
    }

    #[test]
    fn partial_in_item_delimetation_item_undefined_length_returns_incomplete() {
        let bytes = make_sequence_item_undefined_length();
        let mut parser = SequenceItemDataParser::<ExplicitLittleEndian>::new(0xFFFF_FFFF);
        let mut handler = TestHandler::default();
        let attribute = Attribute {
            tag: Tag::new(0x0008, 0x0008),
            vr: None,
            length: 0,
        };
        match parser.parse(&mut handler, &attribute, &bytes[0..13], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 12);
                assert_eq!(result.state, ParseState::Incomplete);
            }
            Err(_error) => panic!("Let's play Global Thermonuclear War"),
        }
    }
}
