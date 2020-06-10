use crate::encoding::Encoding;
use crate::parser::attribute::AttributeParser;
use crate::parser::handler::Handler;
use std::marker::PhantomData;

pub struct ParseError {}

pub struct ParseResult<T: Encoding> {
    pub bytes_consumed: usize,
    pub parser: Box<dyn Parser<T>>,
}

pub trait Parser<T: Encoding> {
    // parses bytes and returns the number consumed and the next Parser
    // or returns an error with the number of bytes consumed
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()>;
}

pub fn parse<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    bytes: &[u8],
    mut parser: Box<dyn Parser<T>>,
) -> Result<usize, (usize, Box<dyn Parser<T>>)> {
    let mut remaining_bytes = bytes;

    while !remaining_bytes.is_empty() {
        match parser.parse(handler, remaining_bytes) {
            Ok(parse_result) => {
                remaining_bytes = &remaining_bytes[parse_result.bytes_consumed..];
                parser = parse_result.parser;
            }
            Err(()) => {
                return Err((remaining_bytes.len(), parser));
            }
        }
    }

    Ok(0)
}

pub fn parse_full<T: 'static + Encoding>(
    callback: &mut dyn Handler,
    bytes: &[u8],
) -> Result<usize, usize> {
    let parser = Box::new(AttributeParser::<T> {
        phantom: PhantomData,
    });
    match parse(callback, bytes, parser) {
        Ok(consumed) => Ok(consumed),
        Err((_bytes_remaining, _parser)) => Err(_bytes_remaining),
    }
}
