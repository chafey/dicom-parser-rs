use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::tag;
use crate::tag::Tag;
use crate::value_parser::sequence_item_data::SequenceItemDataParser;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;
use crate::value_parser::ParseState;
use crate::value_parser::ValueParser;

#[derive(Default)]
pub struct SequenceParser<T: Encoding> {
    parser: Option<Box<dyn ValueParser<T>>>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> ValueParser<T> for SequenceParser<T> {
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

        // iterate over remaining bytes
        while !remaining_bytes.is_empty() {
            match &mut self.parser {
                None => {
                    // read the tag and length or return incomplete if not enough
                    // bytes
                    if remaining_bytes.len() < 8 {
                        return Ok(ParseResult::incomplete(bytes_consumed));
                    }
                    let (tag, length) = parse_tag_and_length::<T>(remaining_bytes);

                    // if we have undefined length, return completed if we have a sequence
                    // delimitation item (which marks the end of the sequence)
                    if attribute.length == 0xFFFF_FFFF && tag == tag::SEQUENCEDELIMITATIONITEM {
                        return Ok(ParseResult::completed(bytes_consumed + 8));
                    }

                    // verify we have a sequence item tag and return error if not
                    if tag != tag::ITEM {
                        return Err(ParseError {
                            reason: "expected Item tag FFFE,E000",
                            position,
                        });
                    }

                    // update internal state
                    bytes_consumed += 8;
                    self.total_bytes_consumed += 8;
                    remaining_bytes = &remaining_bytes[8..];

                    // notify handle that we are starting a new sequence item
                    handler.start_sequence_item(attribute);

                    // create a new SequenceItemDataParser for this sequence item
                    self.parser = Some(Box::new(SequenceItemDataParser::<T>::new(length)));
                }
                Some(parser) => {
                    // we have a parser so forward the remaining bytes to it to be parsed
                    let parse_result = parser.parse(
                        handler,
                        attribute,
                        remaining_bytes,
                        position + bytes_consumed,
                    )?;

                    // update internal state
                    bytes_consumed += parse_result.bytes_consumed;
                    self.total_bytes_consumed += parse_result.bytes_consumed;
                    remaining_bytes = &remaining_bytes[parse_result.bytes_consumed..];

                    // handle parse result
                    match parse_result.state {
                        ParseState::Cancelled => {
                            return Ok(ParseResult::cancelled(bytes_consumed));
                        }
                        ParseState::Incomplete => {
                            return Ok(ParseResult::incomplete(bytes_consumed));
                        }
                        ParseState::Completed => {
                            handler.end_sequence_item(attribute);
                            self.parser = None;
                            continue; // continue if there are more bytes to parse
                        }
                    }
                }
            }
        }

        // if we have a known length and have consumed all the bytes, we are complete,
        // otherwise we are incomplete
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
