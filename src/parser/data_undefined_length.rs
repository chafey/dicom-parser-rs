use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;
use crate::parser::ParseResult;
use crate::parser::Parser;
use std::marker::PhantomData;

#[derive(Default)]
pub struct DataUndefinedLengthParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + Encoding> Parser<T> for DataUndefinedLengthParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
    ) -> Result<ParseResult, ()> {
        // scan for sequence delimitation item
        let data_length = match find_end_of_data::<T>(bytes) {
            Err(()) => {
                return Ok(ParseResult::incomplete(0));
            }
            Ok(data_length) => data_length,
        };

        // notify handler of data
        handler.data(attribute, &bytes[..data_length]);

        // next is attribute parser
        Ok(ParseResult::completed(data_length + 8))
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
