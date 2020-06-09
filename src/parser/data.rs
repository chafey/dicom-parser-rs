use crate::parser::parser::Parser;
use crate::attribute::Attribute;
use crate::byte_parser::ByteParser;
use crate::dataset::Callback;
use crate::parser::attribute::ExplicitAttributeParser;
use std::marker::PhantomData;

pub struct DataParser<T : ByteParser> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>
}

impl <T : ByteParser> DataParser<T>  {
}

impl <T : 'static + ByteParser> Parser<T> for DataParser<T> {

    fn parse(&mut self, callback: &mut dyn Callback, bytes: &[u8]) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        
        if bytes.len() < self.attribute.length {
            return Err(());
        }

        callback.data(&self.attribute, &bytes[..self.attribute.length]);

        let attribute_parser = Box::new(ExplicitAttributeParser::<T>{phantom: PhantomData});
        return Ok((self.attribute.length, attribute_parser))
    }
}