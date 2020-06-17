use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::basic_offset_table::BasicOffsetTableParser;
use crate::parser::pixel_data_fragment::PixelDataFragmentParser;
use crate::parser::ParseError;
use crate::parser::ParseResult;
use crate::parser::ParseState;
use crate::parser::Parser;
use crate::tag;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct EncapsulatedPixelDataParser<T: Encoding> {
    parser: Box<dyn Parser<T>>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> EncapsulatedPixelDataParser<T> {
    pub fn default() -> EncapsulatedPixelDataParser<T> {
        EncapsulatedPixelDataParser {
            parser: Box::new(BasicOffsetTableParser::<T> {
                phantom: PhantomData,
            }),
            total_bytes_consumed: 0,
        }
    }
}

impl<T: 'static + Encoding> Parser<T> for EncapsulatedPixelDataParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        position: usize,
    ) -> Result<ParseResult, ParseError> {
        // iterate over remaining bytes parsing them
        let mut remaining_bytes = bytes;
        let mut bytes_consumed = 0;
        while !remaining_bytes.is_empty() {
            // check for sequence delimeter item
            if remaining_bytes.len() < 8 {
                return Ok(ParseResult::incomplete(bytes_consumed));
            }
            let (tag, _length) = parse_tag_and_length::<T>(remaining_bytes);
            if tag == tag::SEQUENCEDELIMITATIONITEM {
                return Ok(ParseResult::completed(bytes_consumed + 8));
            }

            let parse_result = self.parser.parse(
                handler,
                attribute,
                remaining_bytes,
                position + bytes_consumed,
            )?;

            self.total_bytes_consumed += parse_result.bytes_consumed;
            remaining_bytes = &remaining_bytes[parse_result.bytes_consumed..];
            bytes_consumed += parse_result.bytes_consumed;

            match parse_result.state {
                ParseState::Cancelled => {
                    return Ok(parse_result);
                }
                ParseState::Incomplete => {
                    return Ok(parse_result);
                }
                ParseState::Completed => {
                    self.parser = Box::new(PixelDataFragmentParser::<T> {
                        phantom: PhantomData,
                    });
                }
            };
        }

        Ok(ParseResult::incomplete(bytes_consumed))
    }
}

pub fn parse_tag_and_length<T: Encoding>(bytes: &[u8]) -> (Tag, usize) {
    let tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;
    (tag, length)
}
