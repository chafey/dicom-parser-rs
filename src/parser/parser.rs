use crate::byte_parser::ByteParser;
use crate::dataset::Callback;
use crate::parser::attribute::ExplicitAttributeParser;
use std::marker::PhantomData;

pub trait Parser<T : ByteParser> {
    // parses bytes and returns the number consumed and the next Parser
    fn parse(&mut self, callback: &mut dyn Callback, bytes: &[u8]) -> Result<(usize, Box<dyn Parser<T>>), ()>;
}

pub fn parse<T: 'static + ByteParser>(callback: &mut dyn Callback, bytes: &[u8], mut parser: Box<dyn Parser<T>>) -> Result<(), (usize, Box<dyn Parser<T>>)> {

    let mut remaining_bytes = bytes;

    while remaining_bytes.len() > 0 {
        match parser.parse(callback, remaining_bytes) {
            Ok( (bytes_consumed, next_parser)) => {
                parser = next_parser;
                remaining_bytes = &remaining_bytes[bytes_consumed..];
            },
            Err(()) => {
                 return Err((remaining_bytes.len(), parser));
            }
        }
    }

    Ok(())
}

pub fn parse_full<T: 'static + ByteParser>(callback: &mut dyn Callback, bytes: &[u8]) -> Result<(), usize> {
    let parser = Box::new(ExplicitAttributeParser::<T>{phantom: PhantomData});
    match parse(callback, bytes, parser) {
        Ok(()) => Ok(()),
        Err((_bytes_remaining, _parser)) => Err(_bytes_remaining) 
    }
}