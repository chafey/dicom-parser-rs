use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::data_set::Parser;
use crate::parser::encapsulated_pixel_data::EncapsulatedPixelDataParser;
use crate::parser::handler::Handler;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct BasicOffsetTableParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> BasicOffsetTableParser<T> {}

impl<T: 'static + Encoding> Parser<T> for BasicOffsetTableParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
    ) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        // make sure we have enough length to read item and length
        if bytes.len() < 8 {
            return Err(());
        }

        // Validate the item tag
        let item_tag = Tag::from_bytes::<T>(bytes);
        if item_tag != Tag::new(0xFFFE, 0xE000) {
            return Err(());
        }

        // Read the item length and make sure we have enough bytes for it
        let item_length = T::u32(&bytes[4..8]) as usize;
        if bytes.len() < item_length + 8 {
            return Err(());
        }

        // notify handler of data
        handler.basic_offset_table(&self.attribute, &bytes[8..(8 + item_length)]);

        // read the encapsulated pixel data
        let attribute_parser = Box::new(EncapsulatedPixelDataParser::<T> {
            attribute: self.attribute,
            phantom: PhantomData,
        });
        Ok((item_length + 8, attribute_parser))
    }
}