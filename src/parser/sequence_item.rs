use crate::parser::parser::Parser;
use std::marker::PhantomData;
use crate::byte_parser::ByteParser;
use crate::parser::attribute::ExplicitAttributeParser;
use crate::dataset::Callback;
use crate::parser::parser::parse_full;

pub struct SequenceItemParser<T : ByteParser> {
    pub phantom: PhantomData<T>,
    pub length: usize
}

impl <T : ByteParser> SequenceItemParser<T>  {
}

impl <T : 'static + ByteParser> Parser<T> for SequenceItemParser<T> {

    fn parse(&mut self, callback: &mut dyn Callback, bytes: &[u8]) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        if bytes.len() < self.length {
            return Err(());
        }

        let result = parse_full::<T>(callback, &bytes[0..self.length]);

        //let attribute_parser = Box::new(ExplicitAttributeParser::<T>{phantom: PhantomData});

        return Ok((self.length, attribute_parser))
    }
}