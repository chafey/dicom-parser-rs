use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Control;
use crate::handler::Handler;
use crate::parser::data::DataParser;
use crate::parser::data_undefined_length::DataUndefinedLengthParser;
use crate::parser::encapsulated_pixel_data::EncapsulatedPixelDataParser;
use crate::parser::sequence::SequenceParser;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag;
use crate::tag::Tag;
use crate::vr::VR;

#[derive(Default)]
pub struct AttributeParser<T: Encoding> {
    attribute: Attribute,
    parser: Option<Box<dyn Parser<T>>>,
}

impl<T: 'static + Encoding> AttributeParser<T> {
    pub fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        match &mut self.parser {
            None => {
                let (bytes_consumed, attribute) = match parse_attribute::<T>(bytes) {
                    Ok((bytes_consumed, attribute)) => (bytes_consumed, attribute),
                    Err(()) => {
                        return Ok(ParseResult::incomplete(0));
                    }
                };

                self.attribute = attribute;

                match handler.element(&self.attribute) {
                    Control::Continue => {}
                    Control::Cancel => {
                        return Ok(ParseResult::cancelled(0));
                    }
                }

                let remaining_byes = &bytes[bytes_consumed..];

                self.parser = Some(make_parser::<T>(handler, &attribute, remaining_byes));
                let mut parse_result = self.parser.as_mut().unwrap().parse(
                    handler,
                    &self.attribute,
                    remaining_byes,
                )?;
                parse_result.bytes_consumed += bytes_consumed;
                Ok(parse_result)
            }
            Some(parser) => parser.parse(handler, &self.attribute, bytes),
        }
    }
}

fn make_parser<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    attribute: &Attribute,
    bytes: &[u8],
) -> Box<dyn Parser<T>> {
    if attribute.vr == Some(VR::SQ) {
        handler.start_sequence(attribute);
        Box::new(SequenceParser::<T>::default())
    } else if is_encapsulated_pixel_data(attribute) {
        Box::new(EncapsulatedPixelDataParser::default())
    } else if attribute.length == 0xFFFF_FFFF {
        // TODO: Consider moving sequence parsing into dataundefinedlengthparser
        if is_sequence::<T>(bytes) {
            handler.start_sequence(attribute);
            Box::new(SequenceParser::<T>::default())
        } else {
            Box::new(DataUndefinedLengthParser::<T>::default())
        }
    } else {
        Box::new(DataParser::<T>::default())
    }
}

fn parse_attribute<T: Encoding>(bytes: &[u8]) -> Result<(usize, Attribute), ()> {
    if bytes.len() < 6 {
        return Err(());
    }
    let group = T::u16(&bytes[0..2]);
    let element = T::u16(&bytes[2..4]);
    let tag = Tag::new(group, element);

    let (vr, length, bytes_consumed) = if is_sequence_tag(tag) {
        if bytes.len() < 8 {
            return Err(());
        }
        let length = T::u32(&bytes[4..8]) as usize;
        (None, length, 4)
    } else {
        T::vr_and_length(&bytes)?
    };

    // if we have undefined length, check to make sure we have an additional 8 bytes
    // which will be needed for implicit little endian to detect if this is a sequence
    // item or not.  This check occurs shortly after this function is called
    if length == 0xFFFF_FFFF && bytes.len() < (bytes_consumed + 8) {
        return Err(());
    }

    let attribute = Attribute {
        tag: Tag::new(group, element),
        vr,
        length,
    };
    Ok((bytes_consumed, attribute))
}

fn is_sequence_tag(tag: Tag) -> bool {
    tag.group == 0xFFFE && (tag.element == 0xE000 || tag.element == 0xE00D || tag.element == 0xE0DD)
}

fn is_encapsulated_pixel_data(attribute: &Attribute) -> bool {
    attribute.tag == Tag::new(0x7fe0, 0x0010) && attribute.length == 0xffff_ffff
}

fn is_sequence<T: Encoding>(bytes: &[u8]) -> bool {
    let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
    item_tag == tag::ITEM
}
