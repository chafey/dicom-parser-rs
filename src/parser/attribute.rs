use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::basic_offset_table::BasicOffsetTableParser;
use crate::parser::data::DataParser;
use crate::parser::data_undefined_length::DataUndefinedLengthParser;
use crate::parser::handler::Control;
use crate::parser::handler::Handler;
use crate::parser::sequence::SequenceParser;
use crate::parser::ParseResult;
use crate::parser::Parser;
use crate::tag::Tag;
use crate::vr::VR;
use std::marker::PhantomData;

pub struct AttributeParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + Encoding> Parser<T> for AttributeParser<T> {
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()> {
        parse(handler, bytes)
    }
}

fn parse<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    bytes: &[u8],
) -> Result<ParseResult<T>, ()> {
    if bytes.len() < 6 {
        return Ok(ParseResult::incomplete());
    }

    let (bytes_consumed, attribute) = parse_attribute::<T>(bytes)?;

    match handler.element(&attribute) {
        Control::Continue => {}
        Control::Filter => {
            // TODO: Skip data
        }
        Control::Stop => {
            return Ok(ParseResult::cancelled(0));
        }
    }

    if attribute.vr == Some(VR::SQ) {
        let parser = Box::new(SequenceParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok(ParseResult::partial(bytes_consumed, parser))
    } else if is_encapsulated_pixel_data(&attribute) {
        let parser = Box::new(BasicOffsetTableParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok(ParseResult::partial(bytes_consumed, parser))
    } else if attribute.length == 0xFFFF_FFFF {
        if is_sequence::<T>(&bytes[bytes_consumed..]) {
            let parser = Box::new(SequenceParser::<T> {
                attribute,
                phantom: PhantomData,
            });
            Ok(ParseResult::partial(bytes_consumed, parser))
        } else {
            let parser = Box::new(DataUndefinedLengthParser::<T> {
                phantom: PhantomData,
                attribute,
            });
            Ok(ParseResult::partial(bytes_consumed, parser))
        }
    } else {
        let parser = Box::new(DataParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok(ParseResult::partial(bytes_consumed, parser))
    }
}

fn parse_attribute<T: Encoding>(bytes: &[u8]) -> Result<(usize, Attribute), ()> {
    let group = T::u16(&bytes[0..2]);
    let element = T::u16(&bytes[2..4]);

    let (vr, length, bytes_consumed) = T::vr_and_length(&bytes)?;

    let attribute = Attribute {
        tag: Tag::new(group, element),
        vr,
        length,
        had_unknown_length: false,
    };
    Ok((bytes_consumed, attribute))
}

fn is_encapsulated_pixel_data(attribute: &Attribute) -> bool {
    attribute.tag == Tag::new(0x7fe0, 0x0010) && attribute.length == 0xffff_ffff
}

fn is_sequence<T: Encoding>(bytes: &[u8]) -> bool {
    // peek ahead to see if it looks like a sequence
    if bytes.len() >= 8 {
        let item_tag = Tag::from_bytes::<T>(&bytes[0..4]);
        if item_tag.group == 0xFFFE && item_tag.element == 0xE000 {
            return true;
        }
    }
    false
}
