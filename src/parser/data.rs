use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::attribute::AttributeParser;
use crate::handler::Handler;
use crate::parser::ParseResult;
use crate::parser::Parser;
use std::marker::PhantomData;

pub struct DataParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> DataParser<T> {}

impl<T: 'static + Encoding> Parser<T> for DataParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        // make sure we have enough bytes for this data
        if bytes.len() < self.attribute.length {
            return Ok(ParseResult::incomplete());
        }

        // notify handler of data
        handler.data(&self.attribute, &bytes[..self.attribute.length]);

        // next is attribute parser
        let parser = Box::new(AttributeParser::<T> {
            phantom: PhantomData,
        });
        Ok(ParseResult::partial(self.attribute.length, parser))
    }
}
