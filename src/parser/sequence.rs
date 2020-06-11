use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::stop::StopHandler;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::DataSetParser;
use crate::parser::handler::Handler;
use crate::parser::ParseResult;
use crate::parser::ParseState;
use crate::parser::Parser;
use crate::tag;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct SequenceParser<T: Encoding> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>,
}

impl<T: Encoding> SequenceParser<T> {}

impl<T: 'static + Encoding> Parser<T> for SequenceParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        let mut remaining_bytes = if self.attribute.length == 0xFFFF_FFFF {
            bytes
        } else {
            &bytes[0..self.attribute.length]
        };

        let mut bytes_consumed = 0;

        while !remaining_bytes.is_empty() {
            if bytes.len() < 8 {
                return Ok(ParseResult::incomplete());
            }

            let _sequence_item_length = parse_sequence_item::<T>(remaining_bytes)?;

            remaining_bytes = &remaining_bytes[8..];
            bytes_consumed += 8;

            handler.start_sequence_item(&self.attribute);

            // create a stop handler for the Item Delimitation Item tag so we
            // stop parsing at the end of this sequence item
            let mut sequence_item_handler = StopHandler {
                stop_fn: |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM,
                handler,
            };

            let mut parser = DataSetParser::<T>::new(&mut sequence_item_handler);
            let (parse_state, consumed) = parser.parse(remaining_bytes)?;

            if parse_state == ParseState::Incomplete {
                // FIXME: we fail here because it will currently result in multiple
                // calls to the handler for the same data when the parse is resumed.
                return Err(());
            }

            handler.end_sequence_item(&self.attribute);

            remaining_bytes = &remaining_bytes[consumed..];
            bytes_consumed += consumed;

            // check for end of sequence
            if remaining_bytes.len() < 8 {
                // FIXME: we fail here because it will currently result in multiple
                // calls to the handler for the same data when the parse is resumed.
                return Err(());
            }
            let item_tag = Tag::from_bytes::<T>(&remaining_bytes[0..4]);
            let _item_length = T::u32(&remaining_bytes[4..8]) as usize;
            if item_tag == tag::SEQUENCEDELIMITATIONITEM {
                // end of sequence
                bytes_consumed += 8;
                break;
            }
        }

        let parser = Box::new(AttributeParser::<T> {
            phantom: PhantomData,
        });
        Ok(ParseResult::partial(bytes_consumed, parser))
    }
}

pub fn parse_sequence_item<T: Encoding>(bytes: &[u8]) -> Result<usize, ()> {
    let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;

    if item_tag != tag::ITEM {
        return Err(());
    }

    Ok(length)
}
