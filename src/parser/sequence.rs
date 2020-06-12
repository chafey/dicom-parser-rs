use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::stop::StopHandler;
use crate::handler::Handler;
use crate::parser::attribute::AttributeParser;
use crate::parser::data_set::DataSetParser;
use crate::parser::ParseResult;
use crate::parser::ParseState;
use crate::parser::Parser;
use crate::tag;
use crate::tag::Tag;
use std::marker::PhantomData;

pub struct SequenceParser<T: Encoding> {
    pub attribute: Attribute,
    parser: Option<Box<dyn Parser<T>>>,
    total_bytes_consumed: usize,
    phantom: PhantomData<T>,
}

impl<T: Encoding> SequenceParser<T> {
    pub fn new(attribute: Attribute) -> SequenceParser<T> {
        SequenceParser::<T> {
            attribute,
            parser: None,
            phantom: PhantomData,
            total_bytes_consumed: 0,
        }
    }
}

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
                return Ok(ParseResult::incomplete(0));
            }

            let _sequence_item_length = parse_sequence_item::<T>(remaining_bytes)?;

            remaining_bytes = &remaining_bytes[8..];
            bytes_consumed += 8;
            self.total_bytes_consumed += 8;

            handler.start_sequence_item(&self.attribute);

            // create a stop handler for the Item Delimitation Item tag so we
            // stop parsing at the end of this sequence item
            let mut sequence_item_handler = StopHandler {
                stop_fn: |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM,
                handler,
            };

            self.parser = Some(Box::new(DataSetParser::<T>::default()));
            let result = match &mut self.parser {
                Some(parser) => parser.parse(&mut sequence_item_handler, remaining_bytes)?,
                None => {
                    return Err(()); // not possible...
                }
            };

            bytes_consumed += result.bytes_consumed;

            if result.state == ParseState::Incomplete {
                // FIXME: we fail here because it will currently result in multiple
                // calls to the handler for the same data when the parse is resumed.
                return Ok(ParseResult::incomplete(bytes_consumed));
            }

            handler.end_sequence_item(&self.attribute);

            remaining_bytes = &remaining_bytes[result.bytes_consumed..];

            // check for end of sequence
            if self.attribute.length == 0xFFFF_FFFF {
                // TODO: Skip over item delimeter item
                remaining_bytes = &remaining_bytes[8..];
                bytes_consumed += 8;

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
