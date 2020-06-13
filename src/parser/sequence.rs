use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::stop::StopHandler;
use crate::handler::Handler;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::DataSetParser;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct SequenceParser<T: Encoding> {
    pub attribute: Attribute,
    parser: Option<Box<dyn Parser<T>>>,
    total_bytes_consumed: usize,
    phantom: PhantomData<T>,
    sequence_item_bytes_remaining: usize,
}

impl<T: Encoding> SequenceParser<T> {
    pub fn new(attribute: Attribute) -> SequenceParser<T> {
        SequenceParser::<T> {
            attribute,
            parser: None,
            phantom: PhantomData,
            total_bytes_consumed: 0,
            sequence_item_bytes_remaining: 0,
        }
    }
}

impl<T: 'static + Encoding> Parser<T> for SequenceParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        let mut remaining_bytes =
            if self.attribute.length == 0xFFFF_FFFF || bytes.len() < self.attribute.length {
                bytes
            } else if bytes.len() > (self.attribute.length - self.total_bytes_consumed) {
                &bytes[0..(self.attribute.length - self.total_bytes_consumed)]
            } else {
                &bytes[0..self.attribute.length]
            };

        let mut bytes_consumed = 0;

        // create a stop handler for the Item Delimitation Item tag so we
        // stop parsing at the end of this sequence item
        let mut sequence_item_handler =
            StopHandler::new(handler, |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM);

        while !remaining_bytes.is_empty() {
            if self.parser.is_none() {
                if remaining_bytes.len() < 8 {
                    return Ok(ParseResult::incomplete(bytes_consumed));
                }

                let sequence_item_length = parse_sequence_item::<T>(remaining_bytes)?;
                self.sequence_item_bytes_remaining = sequence_item_length;

                bytes_consumed += 8;
                self.total_bytes_consumed += 8;
                remaining_bytes = &remaining_bytes[8..];

                sequence_item_handler
                    .handler
                    .start_sequence_item(&self.attribute);

                // if we don't have a parser already create one.  we will have one already
                // if we are resuming..
                self.parser = Some(Box::new(DataSetParser::<T>::default()));
            }

            let sequence_item_bytes = if self.sequence_item_bytes_remaining < remaining_bytes.len()
            {
                &remaining_bytes[0..self.sequence_item_bytes_remaining]
            } else {
                remaining_bytes
            };

            let result = match &mut self.parser {
                Some(parser) => parser.parse(&mut sequence_item_handler, sequence_item_bytes)?,
                None => {
                    return Err(());
                    //panic!("impossible")
                }
            };

            self.sequence_item_bytes_remaining -= result.bytes_consumed;
            bytes_consumed += result.bytes_consumed;
            self.total_bytes_consumed += result.bytes_consumed;
            remaining_bytes = &remaining_bytes[result.bytes_consumed..];

            // if we didn't have all expected bytes, return incomplete parse
            if (self.attribute.length != 0xFFFF_FFFF) && (self.sequence_item_bytes_remaining > 0) {
                return Ok(ParseResult::incomplete(bytes_consumed));
            }

            /*if result.state == ParseState::Incomplete {
                // FIXME: we fail here because it will currently result in multiple
                // calls to the handler for the same data when the parse is resumed.
                return Ok(ParseResult::incomplete(bytes_consumed));
            }*/

            sequence_item_handler
                .handler
                .end_sequence_item(&self.attribute);

            // TODO: create parser based on what is next - sequence item, attribute,  end of sequence out of bytes

            // check for end of sequence
            if self.attribute.length == 0xFFFF_FFFF {
                // TODO: Skip over item delimeter item
                bytes_consumed += 8;
                self.total_bytes_consumed += 8;
                remaining_bytes = &remaining_bytes[8..];

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
                self.parser = None;
            }
            self.parser = None;
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

pub fn parse_tag_and_length<T: Encoding>(bytes: &[u8]) -> Result<usize, ()> {
    let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
    let length = T::u32(&bytes[4..8]) as usize;

    if item_tag != tag::ITEM {
        return Err(());
    }

    Ok(length)
}
