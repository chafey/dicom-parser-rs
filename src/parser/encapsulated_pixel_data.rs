use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::ParseResult;
use crate::parser::data_set::Parser;
use crate::parser::handler::Handler;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct EncapsulatedPixelDataParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> EncapsulatedPixelDataParser<T> {}

impl<T: 'static + Encoding> Parser<T> for EncapsulatedPixelDataParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        // read item tag
        if bytes.len() < 4 {
            return Err(());
        }
        let item_tag = Tag::from_bytes::<T>(bytes);

        // check for sequence delimeter item
        if item_tag.group == 0xFFFE && item_tag.element == 0xE0DD {
            let parser = Box::new(AttributeParser::<T> {
                phantom: PhantomData,
            });
            return Ok(ParseResult {
                bytes_consumed: 4,
                parser,
            });
        }

        // check for sequence item
        if item_tag != Tag::new(0xFFFE, 0xE000) {
            return Err(());
        }

        // read item length
        if bytes.len() < 8 {
            return Err(());
        }
        let item_length = T::u32(&bytes[4..8]) as usize;

        // make sure we have enough bytes for the item value
        if bytes.len() < item_length + 8 {
            return Err(());
        }

        // notify handler of data
        handler.pixel_data_fragment(&self.attribute, &bytes[8..(8 + item_length)]);

        // read the encapsulated pixel data
        let parser = Box::new(AttributeParser::<T> {
            phantom: PhantomData,
        });
        Ok(ParseResult {
            bytes_consumed: item_length + 8,
            parser,
        })
    }
}
