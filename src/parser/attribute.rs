use crate::attribute::Attribute;
use crate::encoding::ByteParser;
use crate::handler::Control;
use crate::handler::Handler;
use crate::parser::data::DataParser;
use crate::parser::dataset::Parser;
use crate::parser::sequence::SequenceParser;
use crate::tag::Tag;
use crate::vr::VR;
use std::marker::PhantomData;

pub struct ExplicitAttributeParser<T: ByteParser> {
    pub phantom: PhantomData<T>,
}

impl<T: 'static + ByteParser> Parser<T> for ExplicitAttributeParser<T> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        bytes: &[u8],
    ) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        parse(handler, bytes)
    }
}

fn parse<T: 'static + ByteParser>(
    handler: &mut dyn Handler,
    bytes: &[u8],
) -> Result<(usize, Box<dyn Parser<T>>), ()> {
    if bytes.len() < 6 {
        return Err(());
    }

    let (bytes_consumed, mut attribute) = parse_attribute::<T>(bytes)?;

    if attribute.length == 0xffff_ffff {
        // HACK: update attribute length to the remaining bytes
        attribute.length = bytes.len() - bytes_consumed;
        attribute.had_unknown_length = true;
    }

    match handler.element(&attribute) {
        Control::Element => {}
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
    } else {
        let data_parser = Box::new(DataParser::<T> {
            phantom: PhantomData,
            attribute,
        });
        Ok((bytes_consumed, data_parser))
    }
}

fn parse_attribute<T: ByteParser>(bytes: &[u8]) -> Result<(usize, Attribute), ()> {
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
