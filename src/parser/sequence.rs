use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::stop::StopHandler;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::parse_full;
use crate::parser::data_set::ParseResult;
use crate::parser::data_set::Parser;
use crate::parser::handler::Handler;
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
        let mut remaining_bytes = bytes;

        let mut bytes_consumed = 0;

        while !remaining_bytes.is_empty() {
            parse_sequence_item::<T>(remaining_bytes)?;

            remaining_bytes = &remaining_bytes[8..];
            bytes_consumed += 8;

            handler.start_sequence_item(&self.attribute);

            // create a stop handler for the Item Delimitation Item tag so we
            // stop parsing at the end of this sequence item
            let mut sequence_item_handler = StopHandler {
                stop_fn: |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM,
                handler,
            };

            let consumed = match parse_full::<T>(&mut sequence_item_handler, remaining_bytes) {
                Ok(_consumed) => {
                    // this should never happen since we are expecting to get an error from stopping on the item delimitation item
                    return Err(());
                }
                Err(remaining) => remaining_bytes.len() - remaining + 8,
            };

            handler.end_sequence_item(&self.attribute);

            remaining_bytes = &remaining_bytes[consumed..];
            bytes_consumed += consumed;

            // check for end of sequence
            if remaining_bytes.len() < 8 {
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

        Ok(ParseResult {
            bytes_consumed,
            parser,
        })
    }
}

pub fn parse_sequence_item<T: Encoding>(bytes: &[u8]) -> Result<usize, ()> {
    if bytes.len() < 8 {
        return Err(());
    }
    let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;

    if item_tag != tag::ITEM {
        return Err(());
    }

    Ok(length)
}
