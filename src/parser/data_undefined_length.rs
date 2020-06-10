use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::ParseResult;
use crate::parser::data_set::Parser;
use crate::parser::handler::Handler;
use std::marker::PhantomData;

pub struct DataUndefinedLengthParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> DataUndefinedLengthParser<T> {}

impl<T: 'static + Encoding> Parser<T> for DataUndefinedLengthParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        // scan for sequence delimitation item
        let data_length = find_end_of_data::<T>(bytes)?;

        // notify handler of data
        handler.data(&self.attribute, &bytes[..data_length]);

        // next is attribute parser
        let parser = Box::new(AttributeParser::<T> {
            phantom: PhantomData,
        });
        Ok(ParseResult {
            bytes_consumed: data_length + 8,
            parser,
        })
    }
}

fn find_end_of_data<T: Encoding>(bytes: &[u8]) -> Result<usize, ()> {
    let mut position = 0;
    while position <= bytes.len() - 4 {
        let group = T::u16(&bytes[position..position + 2]);
        position += 2;
        if group == 0xFFFE {
            let element = T::u16(&bytes[position..position + 2]);
            if element == 0xE0DD {
                // TODO: Consider verifying zero length?

                return Ok(position - 2);
            }
        }
    }

    Err(())
}
