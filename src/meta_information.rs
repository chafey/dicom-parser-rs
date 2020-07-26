use crate::attribute::Attribute;
use crate::data_set_parser::parse_full;
use crate::encoding::ExplicitLittleEndian;
use crate::handler::cancel::CancelHandler;
use crate::handler::tee::TeeHandler;
use crate::handler::Handler;
use crate::handler::HandlerResult;
use crate::prefix;
use crate::tag::Tag;
use crate::value_parser::ParseError;
use std::str;

/*
macro_rules! map_string {
    ($group:expr, $element:expr, $property:expr) => {
        if attribute.tag.group == group && attribute.tag.element == element {
            property = str::from_utf8(self.data_buffer).unwrap()
        }
    };
}*/

/// MetaInformation includes the required attributes from the DICOM P10 Header
#[derive(Debug, Default)]
pub struct MetaInformation {
    /// The SOP Class UID
    pub media_storage_sop_class_uid: String,
    /// The SOP Instance UID
    pub media_storage_sop_instance_uid: String,
    /// The Transfer Syntax UID
    pub transfer_syntax_uid: String,
    /// The Implementation Class UID
    pub implementation_class_uid: String,
    /// The offset from the beginning of the file that the DICOM P10 header
    /// ends at
    pub end_position: usize,
}

struct MetaInformationBuilder<'a> {
    pub meta_information: &'a mut MetaInformation,

    // buffer to accumulate data for an attribute
    data_buffer: Vec<u8>,
}

impl Handler for MetaInformationBuilder<'_> {
    fn attribute(
        &mut self,
        _attribute: &Attribute,
        _position: usize,
        _data_offset: usize,
    ) -> HandlerResult {
        self.data_buffer.clear();
        HandlerResult::Continue
    }
    fn data(&mut self, attribute: &Attribute, data: &[u8], complete: bool) {
        if attribute.length == 0 {
            return;
        }
        self.data_buffer.extend_from_slice(data);

        if complete {
            let bytes = if self.data_buffer[attribute.length - 1] != 0 {
                &self.data_buffer
            } else {
                &self.data_buffer[0..(attribute.length - 1)]
            };

            if attribute.tag == Tag::new(0x0002, 0x02) {
                self.meta_information.media_storage_sop_class_uid =
                    String::from(str::from_utf8(&bytes).unwrap());
            } else if attribute.tag == Tag::new(0x0002, 0x0003) {
                self.meta_information.media_storage_sop_instance_uid =
                    String::from(str::from_utf8(&bytes).unwrap());
            } else if attribute.tag == Tag::new(0x0002, 0x0010) {
                self.meta_information.transfer_syntax_uid =
                    String::from(str::from_utf8(&bytes).unwrap());
            } else if attribute.tag == Tag::new(0x0002, 0x0012) {
                self.meta_information.implementation_class_uid =
                    String::from(str::from_utf8(&bytes).unwrap());
            }
        }
    }
}

/// Parses the DICOM P10 Header and returns it as a MetaInformation instance
///
/// # Arguments
///
/// * `bytes` - bytes containg the entire DICOM P10 Header including the
///             preamble
pub fn parse<'a, T: Handler>(
    handler: &'a mut T,
    bytes: &[u8],
) -> Result<MetaInformation, ParseError> {
    // validate that we have a P10 Header Prefix
    prefix::validate(bytes)?;

    // Create a MetaInformationBuilder
    let mut meta_information = MetaInformation::default();
    let mut builder = MetaInformationBuilder {
        meta_information: &mut meta_information,
        data_buffer: vec![],
    };

    // Create a TeeHandler that forwards Handler callbacks to
    // the user supplied Handler and our MetaInformationBuilder
    let mut tee_handler = TeeHandler::default();
    tee_handler.handlers.push(handler);
    tee_handler.handlers.push(&mut builder);

    // create a CancelHandler that will cancel the parse when we see an attribute that has a
    // tag not in group 2 (All meta information tags are group 2)
    let mut handler = CancelHandler::new(&mut tee_handler, |x: &Attribute| x.tag.group != 2);

    // attempt to fully parse the p10 header.  _cancelled will typically be true.
    // If _cancelled is false, we may  not have a complete p10 header - we can't
    // distinguish between these two until we validate the contents (which we do
    // afterwards)
    let (bytes_consumed, _cancelled) =
        parse_full::<ExplicitLittleEndian>(&mut handler, &bytes[132..], 132)?;

    // calculate the end position of the p10 header by adding the prefix length
    // to the number of bytes consumed parsing the meta information
    meta_information.end_position = 132 + bytes_consumed;

    Ok(meta_information)
}

#[cfg(test)]
pub mod tests {
    use super::parse;
    use crate::test::tests::TestHandler;

    fn make_preamble_and_prefix() -> Vec<u8> {
        let mut bytes = vec![];
        bytes.resize(132, 0);
        bytes[128] = 'D' as u8;
        bytes[129] = 'I' as u8;
        bytes[130] = 'C' as u8;
        bytes[131] = 'M' as u8;

        bytes
    }

    pub fn make_p10_header() -> Vec<u8> {
        let mut bytes = make_preamble_and_prefix();
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x00, 0x00, b'U', b'L', 4, 0, 0, 0, 0, 0]);
        bytes.extend_from_slice(&vec![
            0x02, 0x00, 0x01, 0x00, b'O', b'B', 0, 0, 2, 0, 0, 0, 0, 1,
        ]);
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x02, 0x00, b'U', b'I', 2, 0, b'1', 0]);
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x03, 0x00, b'U', b'I', 2, 0, b'2', 0]);
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x10, 0x00, b'U', b'I', 2, 0, b'3', 0]);
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x12, 0x00, b'U', b'I', 2, 0, b'4', 0]);

        let length = bytes.len() as u32;
        bytes[140] = (length & 0xff) as u8;
        bytes[141] = (length >> 8 & 0xff) as u8;

        bytes
    }

    #[test]
    fn valid_meta_information() {
        let bytes = make_p10_header();
        let mut handler = TestHandler::default();
        //handler.print = true;
        match parse(&mut handler, &bytes) {
            Ok(_meta) => {
                //println!("{:?}", _meta);
                //assert_eq!(meta.data_set.attributes.len(), 6);
            }
            Err(_parse_error) => {
                assert!(false);
            }
        };
    }
}
