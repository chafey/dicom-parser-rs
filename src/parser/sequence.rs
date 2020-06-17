use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::sequence_item_data::SequenceItemDataParser;
use crate::parser::ParseError;
use crate::parser::ParseResult;
use crate::parser::ParseState;
use crate::parser::Parser;
use crate::tag;
use crate::tag::Tag;

#[derive(Default)]
pub struct SequenceParser<T: Encoding> {
    parser: Option<Box<dyn Parser<T>>>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> Parser<T> for SequenceParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        position: usize,
    ) -> Result<ParseResult, ParseError> {
        // if we have a known length, only parse the bytes we know we have
        let mut remaining_bytes = if attribute.length == 0xFFFF_FFFF
            || bytes.len() < (attribute.length - self.total_bytes_consumed)
        {
            bytes
        } else {
            &bytes[0..(attribute.length - self.total_bytes_consumed)]
        };

        let mut bytes_consumed = 0;

        while !remaining_bytes.is_empty() {
            match &mut self.parser {
                None => {
                    if remaining_bytes.len() < 8 {
                        return Ok(ParseResult::incomplete(bytes_consumed));
                    }
                    let (tag, length) = parse_tag_and_length::<T>(remaining_bytes);

                    // if we have undefined length, check for sequence delimitation item
                    if attribute.length == 0xFFFF_FFFF && tag == tag::SEQUENCEDELIMITATIONITEM {
                        return Ok(ParseResult::completed(bytes_consumed + 8));
                    }

                    // verify we have a sequence item and return error if not
                    if tag != tag::ITEM {
                        return Err(ParseError {
                            reason: "expected Item tag FFFE,E000",
                            position,
                        });
                    }

                    bytes_consumed += 8;
                    self.total_bytes_consumed += 8;
                    remaining_bytes = &remaining_bytes[8..];

                    handler.start_sequence_item(attribute);

                    self.parser = Some(Box::new(SequenceItemDataParser::<T>::new(length)));
                }
                Some(parser) => {
                    let parse_result = parser.parse(
                        handler,
                        attribute,
                        remaining_bytes,
                        position + bytes_consumed,
                    )?;

                    bytes_consumed += parse_result.bytes_consumed;
                    self.total_bytes_consumed += parse_result.bytes_consumed;
                    remaining_bytes = &remaining_bytes[parse_result.bytes_consumed..];

                    match parse_result.state {
                        ParseState::Cancelled => {
                            self.parser = None;
                            return Ok(ParseResult::cancelled(bytes_consumed));
                        }
                        ParseState::Incomplete => {
                            return Ok(ParseResult::incomplete(bytes_consumed));
                        }
                        ParseState::Completed => {
                            // this is what we expect in normal happy path
                            handler.end_sequence_item(attribute);
                            self.parser = None;
                            continue;
                        }
                    }
                }
            }
        }

        if attribute.length == 0xFFFF_FFFF || self.total_bytes_consumed < attribute.length {
            Ok(ParseResult::incomplete(bytes_consumed))
        } else {
            handler.end_sequence(attribute);
            Ok(ParseResult::completed(bytes_consumed))
        }
    }
}

pub fn parse_sequence_item<T: Encoding>(bytes: &[u8]) -> Result<usize, ()> {
    let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;

    if item_tag != tag::ITEM {
        return Err(());
    }

    Ok(length)
}

pub fn parse_tag_and_length<T: Encoding>(bytes: &[u8]) -> (Tag, usize) {
    let tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;
    (tag, length)
}
