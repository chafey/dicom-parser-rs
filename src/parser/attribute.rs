use crate::parser::parser::Parser;
use crate::byte_parser::ByteParser;
use crate::vr::VR;
use crate::tag::Tag;
use crate::attribute::Attribute;
use crate::parser::data::DataParser;
use std::marker::PhantomData;
use crate::dataset::Callback;
use crate::dataset::Control;
use crate::parser::sequence::SequenceParser;

pub struct ExplicitAttributeParser<T : ByteParser> {
    pub phantom: PhantomData<T>
}

impl <T : 'static + ByteParser> Parser<T> for ExplicitAttributeParser<T> {

    fn parse(&mut self, callback: &mut dyn Callback, bytes: &[u8]) -> Result<(usize, Box<dyn Parser<T>>), ()> {
        if bytes.len() < 6 {
            return Err(());
        }

        let (bytes_consumed, mut attribute) = parse_attribute::<T>(bytes)?;

        if attribute.length == 0xffffffff {
            // HACK: update attribute length to the remaining bytes
            attribute.length = bytes.len() - bytes_consumed;
            attribute.had_unknown_length = true;
        }

        match callback.element(&attribute) {
            Control::Element => {},
            Control::Data => {},
            Control::Stop => {return Err(());},
        }

        if attribute.vr == Some(VR::SQ) {
            let data_parser = Box::new(SequenceParser::<T>{phantom: PhantomData, attribute});
            Ok((bytes_consumed, data_parser))

        } else {
            let data_parser = Box::new(DataParser::<T>{phantom: PhantomData, attribute});
            Ok((bytes_consumed, data_parser))
        }
    }
}

fn parse_attribute<T:ByteParser>(bytes:&[u8]) -> Result<(usize, Attribute), ()> {
    let group = T::u16(&bytes[0..2]);
    let element = T::u16(&bytes[2..4]);
    let vr = VR::from_bytes(&bytes[4..6]);
    let (bytes_consumed, length) = if vr.explicit_length_is_u32() {
        if bytes.len() < 12 {
            return Err(());
        }
        (12, T::u32(&bytes[8..12]) as usize)
    } else {
        if bytes.len() < 8 {
            return Err(());
        }
        (8, T::u16(&bytes[6..8]) as usize)
    };
    let attribute = Attribute {
        tag : Tag::new(group, element),
        vr: Some(vr),
        length,
        had_unknown_length: false
    };
    Ok((bytes_consumed, attribute))
}
