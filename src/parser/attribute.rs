use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::parser::basic_offset_table::BasicOffsetTableParser;
use crate::parser::data::DataParser;
use crate::parser::data_undefined_length::DataUndefinedLengthParser;
use crate::parser::data_set::Parser;
use crate::parser::handler::Control;
use crate::parser::handler::Handler;
use crate::parser::sequence::SequenceParser;
use crate::tag::Tag;
use crate::vr::VR;
use std::marker::PhantomData;
use crate::parser::sequence_undefined_length::SequenceUndefinedLengthParser;

pub struct AttributeParser<T: Encoding> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + Encoding> Parser<T> for AttributeParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
    ) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        parse(handler, bytes)
    }
}

fn parse<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    bytes: &[u8],
) -> Result<(usize, Box<dyn Parser<T>>), ()> {
    if bytes.len() < 6 {
        return Err(());
    }

    let (bytes_consumed, attribute) = parse_attribute::<T>(bytes)?;

    match handler.element(&attribute) {
        Control::Element => {
            // TODO: Skip data
        }
        Control::Data => {}
        Control::Stop => {
            return Err(());
        }
    }

    if attribute.vr == Some(VR::SQ) {
        let data_parser = Box::new(SequenceParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok((bytes_consumed, data_parser))
    } else if is_encapsulated_pixel_data(&attribute) {
        let data_parser = Box::new(BasicOffsetTableParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok((bytes_consumed, data_parser))
    } else if attribute.length == 0xFFFF_FFFF {
        if is_sequence::<T>(&bytes[bytes_consumed..]) {
            let parser = Box::new(SequenceUndefinedLengthParser::<T> {
                attribute: attribute,
                phantom: PhantomData,
            });
            return Ok((bytes_consumed, parser));
        } else {
            let data_parser = Box::new(DataUndefinedLengthParser::<T> {
                phantom: PhantomData,
                attribute,
            });
            Ok((bytes_consumed, data_parser))
        }
    } else {
        let data_parser = Box::new(DataParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok((bytes_consumed, data_parser))
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
    return false;
}
