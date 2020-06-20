use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::handler::HandlerResult;
use crate::tag;
use crate::tag::Tag;
use crate::value_parser::data::DataParser;
use crate::value_parser::data_undefined_length::DataUndefinedLengthParser;
use crate::value_parser::encapsulated_pixel_data::EncapsulatedPixelDataParser;
use crate::value_parser::sequence::SequenceParser;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;
use crate::value_parser::ValueParser;
use crate::vr::VR;

#[derive(Default)]
pub struct AttributeParser<T: Encoding> {
    attribute: Attribute,
    parser: Option<Box<dyn ValueParser<T>>>,
}

impl<T: 'static + Encoding> AttributeParser<T> {
    pub fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
        bytes_from_beginning: usize,
    ) -> Result<ParseResult, ParseError> {
        match &mut self.parser {
            None => {
                // try to parse the attribute
                let (bytes_consumed, attribute) = match parse_attribute::<T>(bytes) {
                    Ok((bytes_consumed, attribute)) => (bytes_consumed, attribute),
                    Err(()) => {
                        // not enough bytes to parse the attribute so return incomplete
                        return Ok(ParseResult::incomplete(0));
                    }
                };

                self.attribute = attribute;

                // notify the handler of the attribute and return cancelled if the handler
                // cancels it
                match handler.attribute(&self.attribute, bytes_from_beginning, bytes_consumed) {
                    HandlerResult::Continue => {}
                    HandlerResult::Cancel => {
                        return Ok(ParseResult::cancelled(0));
                    }
                }

                // update internal state
                let data_position = bytes_from_beginning + bytes_consumed;
                let remaining_bytes = &bytes[bytes_consumed..];

                // if we have a known length, just get the value field bytes
                let value_bytes = if attribute.length == 0xFFFF_FFFF
                    || remaining_bytes.len() < attribute.length
                {
                    remaining_bytes
                } else {
                    &remaining_bytes[0..attribute.length]
                };

                // create the appropriate value parser for this attribute.
                self.parser = Some(make_parser::<T>(handler, &attribute, value_bytes));

                // parse the value bytes
                let mut parse_result = self.parser.as_mut().unwrap().parse(
                    handler,
                    &self.attribute,
                    value_bytes,
                    data_position,
                )?;

                // add in the size of the attribute tag/vr/length to the bytes consumed and
                // return the parse result
                parse_result.bytes_consumed += bytes_consumed;
                Ok(parse_result)
            }
            Some(parser) => parser.parse(handler, &self.attribute, bytes, bytes_from_beginning),
        }
    }
}

fn make_parser<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    attribute: &Attribute,
    bytes: &[u8],
) -> Box<dyn ValueParser<T>> {
    if attribute.vr == Some(VR::SQ) {
        handler.start_sequence(attribute);
        Box::new(SequenceParser::<T>::default())
    } else if is_encapsulated_pixel_data(attribute) {
        Box::new(EncapsulatedPixelDataParser::default())
    } else if attribute.length == 0xFFFF_FFFF {
        // TODO: Consider moving sequence parsing into dataundefinedlengthparser
        if is_sequence::<T>(bytes) {
            handler.start_sequence(attribute);
            Box::new(SequenceParser::<T>::default())
        } else {
            Box::new(DataUndefinedLengthParser::<T>::default())
        }
    } else {
        Box::new(DataParser::<T>::default())
    }
}

fn parse_attribute<T: Encoding>(bytes: &[u8]) -> Result<(usize, Attribute), ()> {
    if bytes.len() < 6 {
        return Err(());
    }
    let group = T::u16(&bytes[0..2]);
    let element = T::u16(&bytes[2..4]);
    let tag = Tag::new(group, element);

    let (vr, length, bytes_consumed) = if is_sequence_tag(tag) {
        if bytes.len() < 8 {
            return Err(());
        }
        let length = T::u32(&bytes[4..8]) as usize;
        (None, length, 4)
    } else {
        T::vr_and_length(&bytes)?
    };

    // if we have undefined length, check to make sure we have an additional 8 bytes
    // which will be needed for implicit little endian to detect if this is a sequence
    // item or not.  This check occurs shortly after this function is called
    if length == 0xFFFF_FFFF && bytes.len() < (bytes_consumed + 8) {
        return Err(());
    }

    let attribute = Attribute {
        tag: Tag::new(group, element),
        vr,
        length,
    };
    Ok((bytes_consumed, attribute))
}

fn is_sequence_tag(tag: Tag) -> bool {
    tag.group == 0xFFFE && (tag.element == 0xE000 || tag.element == 0xE00D || tag.element == 0xE0DD)
}

fn is_encapsulated_pixel_data(attribute: &Attribute) -> bool {
    attribute.tag == Tag::new(0x7fe0, 0x0010) && attribute.length == 0xffff_ffff
}

fn is_sequence<T: Encoding>(bytes: &[u8]) -> bool {
    let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
    item_tag == tag::ITEM
}

#[cfg(test)]
mod tests {

    use super::AttributeParser;
    use crate::encoding::ExplicitLittleEndian;
    use crate::test::tests::TestHandler;
    use crate::value_parser::ParseState;

    fn make_encapsulated_pixel_data_empty_bot() -> Vec<u8> {
        let mut bytes = vec![];
        // Tag/VR/Length
        bytes.extend_from_slice(&vec![
            0xE0, 0x7F, 0x10, 0x00, b'O', b'B', 0, 0, 0xFF, 0xFF, 0xFF, 0xFF,
        ]);
        // Basic Offset Table (Empty)
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0x00, 0xE0, 0, 0, 0, 0]);
        // Fragment #1 (250 zeros)
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0x00, 0xE0, 250, 0, 0, 0]);
        bytes.extend_from_slice(&vec![0; 250]);
        // end with sequence item delimeter
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0xDD, 0xE0, 0, 0, 0, 0]);

        bytes
    }

    #[test]
    fn full_parse_completes() {
        let mut parser = AttributeParser::<ExplicitLittleEndian>::default();
        let mut handler = TestHandler::default();
        let bytes = make_encapsulated_pixel_data_empty_bot();

        match parser.parse(&mut handler, &bytes[..], 0) {
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
    fn streaming_parse_completes() {
        let mut parser = AttributeParser::<ExplicitLittleEndian>::default();
        let mut handler = TestHandler::default();
        //handler.print = true;
        let bytes = make_encapsulated_pixel_data_empty_bot();

        match parser.parse(&mut handler, &bytes[0..100], 0) {
            Ok(result) => {
                assert_eq!(result.bytes_consumed, 100);
                assert_eq!(result.state, ParseState::Incomplete);
            }
            Err(_error) => {
                assert!(false); // should not happen
            }
        };
        match parser.parse(&mut handler, &bytes[100..], 0) {
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
