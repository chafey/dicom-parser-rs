use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::ParseError;
use crate::parser::ParseResult;
use crate::parser::Parser;
use std::marker::PhantomData;

#[derive(Default)]
pub struct DataParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + Encoding> Parser<T> for DataParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        _position: usize,
    ) -> Result<ParseResult, ParseError> {
        // make sure we have enough bytes for this data
        if bytes.len() < attribute.length {
            return Ok(ParseResult::incomplete(0));
        }

        // notify handler of data
        handler.data(attribute, &bytes[..attribute.length]);

        // next is attribute parser
        Ok(ParseResult::completed(attribute.length))
    }
}
