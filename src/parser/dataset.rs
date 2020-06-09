use crate::encoding::Encoding;
use crate::parser::handler::Handler;
use crate::parser::attribute::ExplicitAttributeParser;
use std::marker::PhantomData;

pub trait Parser<T: Encoding> {
    // parses bytes and returns the number consumed and the next Parser
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
    ) -> Result<(usize, Box<dyn Parser<T>>), ()>;
}

pub fn parse<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    bytes: &[u8],
    mut parser: Box<dyn Parser<T>>,
) -> Result<(), (usize, Box<dyn Parser<T>>)> {
    let mut remaining_bytes = bytes;

    while !remaining_bytes.is_empty() {
        match parser.parse(handler, remaining_bytes) {
            Ok((bytes_consumed, next_parser)) => {
                parser = next_parser;
                remaining_bytes = &remaining_bytes[bytes_consumed..];
            }
            Err(()) => {
                return Err((remaining_bytes.len(), parser));
            }
        }
    }

    Ok(())
}

pub fn parse_full<T: 'static + Encoding>(
    callback: &mut dyn Handler,
    bytes: &[u8],
) -> Result<(), usize> {
    let parser = Box::new(ExplicitAttributeParser::<T> {
        phantom: PhantomData,
    });
    match parse(callback, bytes, parser) {
        Ok(()) => Ok(()),
        Err((_bytes_remaining, _parser)) => Err(_bytes_remaining),
    }
}
