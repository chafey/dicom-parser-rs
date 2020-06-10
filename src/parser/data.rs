use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::ParseResult;
use crate::parser::data_set::Parser;
use crate::parser::handler::Handler;
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
            return Err(());
        }

        // notify handler of data
        handler.data(&self.attribute, &bytes[..self.attribute.length]);

        // next is attribute parser
        let parser = Box::new(AttributeParser::<T> {
            phantom: PhantomData,
        });
        Ok(ParseResult {
            bytes_consumed: self.attribute.length,
            parser,
        })
    }
}
