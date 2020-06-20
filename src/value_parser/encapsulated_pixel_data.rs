use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::tag;
use crate::tag::Tag;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;
use crate::value_parser::ValueParser;
use std::marker::PhantomData;

pub struct EncapsulatedPixelDataParser<T: Encoding> {
    phantom: PhantomData<T>,
    total_bytes_consumed: usize,
    remaining_byte_count: usize,
    item_number: usize,
}

impl<T: 'static + Encoding> EncapsulatedPixelDataParser<T> {
    pub fn default() -> EncapsulatedPixelDataParser<T> {
        EncapsulatedPixelDataParser {
            phantom: PhantomData,
            total_bytes_consumed: 0,
            remaining_byte_count: 0,
            item_number: 0,
        }
    }
}

impl<T: 'static + Encoding> ValueParser<T> for EncapsulatedPixelDataParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        position: usize,
    ) -> Result<ParseResult, ParseError> {
        let mut remaining_bytes = bytes;
        let mut bytes_consumed = 0;

        // iterate over remaining bytes parsing them
        while !remaining_bytes.is_empty() {
            // if no bytes remaining, we are at a new item so read the
            // tag and length
            if self.remaining_byte_count == 0 {
                // read tag and length
                if remaining_bytes.len() < 8 {
                    return Ok(ParseResult::incomplete(bytes_consumed));
                }
                let (tag, length) = parse_tag_and_length::<T>(remaining_bytes);
                self.total_bytes_consumed += 8;
                bytes_consumed += 8;
                remaining_bytes = &remaining_bytes[8..];

                // if sequence delimtation item, we are done
                if tag == tag::SEQUENCEDELIMITATIONITEM {
                    return Ok(ParseResult::completed(bytes_consumed));
                }

                // make sure we have a sequence item
                if tag != tag::ITEM {
                    return Err(ParseError {
                        reason: "expected Item tag FFFE,E000",
                        position: position + bytes_consumed,
                    });
                }

                // make sure item length is not undefined
                if length == 0xFFFF_FFFF {
                    return Err(ParseError {
                        reason: "expected defined length",
                        position: position + bytes_consumed,
                    });
                }

                // set remaining_byte_count so we know how much data to stream
                // to the handler
                self.remaining_byte_count = length;
            }

            // get slice of bytes based on how many we have already parsed for this value field
            let value_bytes = if remaining_bytes.len() > self.remaining_byte_count {
                &remaining_bytes[..self.remaining_byte_count]
            } else {
                remaining_bytes
            };

            // invoke appropriate function on handler (basic offset table is always first)
            let complete = self.remaining_byte_count == value_bytes.len();
            if self.item_number == 0 {
                handler.basic_offset_table(attribute, value_bytes, complete);
            } else {
                handler.pixel_data_fragment(attribute, self.item_number, value_bytes, complete);
            }

            // update counters
            self.total_bytes_consumed += value_bytes.len();
            self.remaining_byte_count -= value_bytes.len();
            bytes_consumed += value_bytes.len();
            remaining_bytes = &remaining_bytes[value_bytes.len()..];

            // if we finished this item, increment item_number
            if self.remaining_byte_count == 0 {
                self.item_number += 1;
            }
        }

        Ok(ParseResult::incomplete(bytes_consumed))
    }
}

pub fn parse_tag_and_length<T: Encoding>(bytes: &[u8]) -> (Tag, usize) {
    let tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;
    (tag, length)
}

#[cfg(test)]
mod tests {

    use super::EncapsulatedPixelDataParser;
    use crate::attribute::Attribute;
    use crate::encoding::ExplicitLittleEndian;
    use crate::tag::Tag;
    use crate::test::tests::TestHandler;
    use crate::value_parser::ParseState;
    use crate::value_parser::ValueParser;
    use crate::vr::VR;

    fn make_encapsulated_pixel_data_value_with_empty_bot() -> Vec<u8> {
        let mut bytes = vec![];
        // Basic Offset Table (Empty)
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0x00, 0xE0, 0, 0, 0, 0]);
        // Fragment #1 (250 zeros)
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0x00, 0xE0, 250, 0, 0, 0]);
        bytes.extend_from_slice(&vec![0; 250]);
        // Fragment #2 (150 zeros)
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0x00, 0xE0, 150, 0, 0, 0]);
        bytes.extend_from_slice(&vec![0; 150]);
        // end with sequence item delimeter
        bytes.extend_from_slice(&vec![0xFE, 0xFF, 0xDD, 0xE0, 0, 0, 0, 0]);

        bytes
    }

    #[test]
    fn full_parse_completes() {
        let mut parser = EncapsulatedPixelDataParser::<ExplicitLittleEndian>::default();
        let mut handler = TestHandler::default();
        let bytes = make_encapsulated_pixel_data_value_with_empty_bot();
        let mut attribute = Attribute::default();
        attribute.tag = Tag::new(0x7fe0, 0x0010);
        attribute.vr = Some(VR::OB);
        attribute.length = 0xFFFF_FFFF;

        match parser.parse(&mut handler, &attribute, &bytes[..], 0) {
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
        let mut parser = EncapsulatedPixelDataParser::<ExplicitLittleEndian>::default();
        let mut handler = TestHandler::default();
        //handler.print = true;
        let bytes = make_encapsulated_pixel_data_value_with_empty_bot();
        let mut attribute = Attribute::default();
        attribute.tag = Tag::new(0x7fe0, 0x0010);
        attribute.vr = Some(VR::OB);
        attribute.length = 0xFFFF_FFFF;

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
