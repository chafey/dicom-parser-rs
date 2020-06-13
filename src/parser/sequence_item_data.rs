use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::stop::StopHandler;
use crate::handler::Handler;
use crate::parser::data_set::DataSetParser;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag;

pub struct SequenceItemDataParser<T: Encoding> {
    item_length: usize,
    parser: DataSetParser<T>,
    total_bytes_consumed: usize,
}

impl<T: 'static + Encoding> SequenceItemDataParser<T> {
    pub fn new(item_length: usize) -> SequenceItemDataParser<T> {
        SequenceItemDataParser {
            item_length,
            parser: DataSetParser::<T>::default(),
            total_bytes_consumed: 0,
        }
    }
}

impl<T: 'static + Encoding> Parser<T> for SequenceItemDataParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        // if we have a known length, only parse the bytes we know we have
        let remaining_bytes = if self.item_length != 0xFFFF_FFFF {
            bytes
        } else {
            &bytes[0..(self.item_length - self.total_bytes_consumed)]
        };

        let mut sequence_item_handler =
            StopHandler::new(handler, |x: &Attribute| x.tag == tag::ITEMDELIMITATIONITEM);

        let parse_result = self
            .parser
            .parse(&mut sequence_item_handler, remaining_bytes)?;

        self.total_bytes_consumed += parse_result.bytes_consumed;

        if sequence_item_handler.stopped || self.total_bytes_consumed == self.item_length {
            Ok(ParseResult::completed(parse_result.bytes_consumed))
        } else {
            Ok(parse_result)
        }
    }
}
