use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::attribute::ExplicitAttributeParser;
use crate::parser::dataset::Parser;
use crate::parser::handler::Handler;
use std::marker::PhantomData;

pub struct DataParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> DataParser<T> {}

impl<T: 'static + Encoding> Parser<T> for DataParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
    ) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        if bytes.len() < self.attribute.length {
            return Err(());
        }

        handler.data(&self.attribute, &bytes[..self.attribute.length]);

        let attribute_parser = Box::new(ExplicitAttributeParser::<T> {
            phantom: PhantomData,
        });
        Ok((self.attribute.length, attribute_parser))
    }
}
