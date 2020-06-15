use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::attribute::AttributeParser;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct EncapsulatedPixelDataParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> EncapsulatedPixelDataParser<T> {}

impl<T: 'static + Encoding> Parser<T> for EncapsulatedPixelDataParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        // read item tag and length
        if bytes.len() < 8 {
            return Ok(ParseResult::incomplete(0));
        }
        let item_tag = Tag::from_bytes::<T>(bytes);
        let item_length = T::u32(&bytes[4..8]) as usize;

        // check for sequence delimeter item
        if item_tag.group == 0xFFFE && item_tag.element == 0xE0DD {
            let parser = Box::new(AttributeParser::<T> {
                phantom: PhantomData,
            });
            return Ok(ParseResult::partial(8, parser));
        }

        // check for sequence item
        if item_tag != Tag::new(0xFFFE, 0xE000) {
            return Err(());
        }

        // make sure we have enough bytes for the item value
        if bytes.len() < item_length + 8 {
            return Ok(ParseResult::incomplete(0));
        }

        // notify handler of data
        handler.pixel_data_fragment(&self.attribute, &bytes[8..(8 + item_length)]);

        /*if bytes.len() == 8 + item_length {
            return Ok(ParseResult::completed(8 + item_length, Box::new(AttributeParser::<T> {
                phantom: PhantomData,
            })));
        }*/

        // read the encapsulated pixel data
        let parser = Box::new(EncapsulatedPixelDataParser::<T> {
            attribute: self.attribute,
            phantom: PhantomData,
        });

        Ok(ParseResult::partial(8 + item_length, parser))
    }
}
