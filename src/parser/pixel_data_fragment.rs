use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct PixelDataFragmentParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + Encoding> Parser<T> for PixelDataFragmentParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
    ) -> Result<ParseResult, ()> {
        // read item tag and length
        if bytes.len() < 8 {
            return Ok(ParseResult::incomplete(0));
        }
        let item_tag = Tag::from_bytes::<T>(bytes);
        let item_length = T::u32(&bytes[4..8]) as usize;

        // check for sequence item
        if item_tag != Tag::new(0xFFFE, 0xE000) {
            return Err(());
        }

        // make sure we have enough bytes for the item value
        if bytes.len() < item_length + 8 {
            return Ok(ParseResult::incomplete(0));
        }

        // notify handler of data
        handler.pixel_data_fragment(attribute, &bytes[8..(8 + item_length)]);

        Ok(ParseResult::completed(8 + item_length))
    }
}
