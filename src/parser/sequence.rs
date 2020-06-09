use crate::parser::parser::Parser;
use std::marker::PhantomData;
use crate::attribute::Attribute;
use crate::byte_parser::ByteParser;
use crate::dataset::Callback;
use crate::sequence_item;
use crate::parser::parser::parse_full;
use crate::parser::attribute::ExplicitAttributeParser;

pub struct SequenceParser<T : ByteParser> {
    pub attribute: Attribute,
    pub phantom: PhantomData<T>
}

impl <T : ByteParser> SequenceParser<T>  {
}

impl <T : 'static + ByteParser> Parser<T> for SequenceParser<T> {

    fn parse(&mut self, callback: &mut dyn Callback, bytes: &[u8]) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        // make sure we have enough bytes to parse the entire sequence
        if bytes.len() < self.attribute.length {
            return  Err(());
        }

        let mut remaining_bytes = &bytes[0..self.attribute.length];

        while remaining_bytes.len() > 0 {

            let sequence_item_length = sequence_item::read(&bytes[0..8])?;

            if remaining_bytes.len() < sequence_item_length {
                return Err(());
            }

            let sequence_item_bytes = &remaining_bytes[8..(8 + sequence_item_length)];

            callback.start_sequence_item(&self.attribute);

            match parse_full::<T>(callback, sequence_item_bytes) {
                Ok(()) => {},
                Err(_remaining) => {
                    // TODO: Handle this unrecoverable error more gracefully
                    panic!("unexpected eof parsing sequence item");
                }
            }

            callback.end_sequence_item(&self.attribute);

            remaining_bytes = &remaining_bytes[(8 + sequence_item_length)..];
        }

        let attribute_parser = Box::new(ExplicitAttributeParser::<T>{phantom: PhantomData});
 
        Ok((self.attribute.length, attribute_parser))
    }
}