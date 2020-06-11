use crate::encoding::Encoding;
use crate::parser::handler::Handler;
use crate::parser::ParseResult;
use crate::parser::Parser;
use std::marker::PhantomData;

pub struct StoppedParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> StoppedParser<T> {}

impl<T: 'static + Encoding> Parser<T> for StoppedParser<T> {
    fn parse(&mut self, _handler: &mut dyn Handler, _bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        Err(())
    }
}
