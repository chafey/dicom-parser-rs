use crate::attribute::Attribute;
use crate::data_set::DataSet;
use crate::data_set_parser::parse_full;
use crate::encoding::ExplicitLittleEndian;
use crate::handler::cancel::CancelHandler;
use crate::handler::data_set::DataSetHandler;
use crate::handler::tee::TeeHandler;
use crate::handler::Handler;
use crate::prefix;
use crate::tag::Tag;
use crate::value_parser::ParseError;
use std::str;

/// MetaInformation includes the required attributes from the DICOM P10 Header
#[derive(Debug)]
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
    /// DataSet instance that contains all attributes and data parsed from the
    /// P10 Header including non required attributes
    pub data_set: DataSet,
}

/// Helper function to find the index of a given attribute in an array of Attributes
fn find_element_index(attributes: &[Attribute], tag: Tag) -> Result<usize, ParseError> {
    for (index, attribute) in attributes.iter().enumerate() {
        if attribute.tag == tag {
            return Ok(index);
        }
    }
    Err(ParseError {
        reason: "missing required tag",
        position: 132,
    })
}

/// Helper function to return the data of an element in a DataSet as a UTF8 string.
fn get_element(dataset: &DataSet, tag: Tag) -> Result<String, ParseError> {
    let index = find_element_index(&dataset.attributes, tag)?;
    let attribute = &dataset.attributes[index];
    let bytes = if dataset.data[index][attribute.length - 1] != 0 {
        &dataset.data[index]
    } else {
        &dataset.data[index][0..(attribute.length - 1)]
    };

    let value = str::from_utf8(bytes).unwrap();
    Ok(String::from(value))
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

    // create DataSetHandler to accumulate the attributes found during the parse
    let mut data_set_handler = DataSetHandler::default();

    let mut tee_handler = TeeHandler::default();
    tee_handler.handlers.push(&mut data_set_handler);
    tee_handler.handlers.push(handler);

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
    let end_position = 132 + bytes_consumed;

    // extract the valules of the required attributes.  Note that get_element()
    // returns an error if a required attribute is not found
    let data_set = data_set_handler.dataset;
    let meta = MetaInformation {
        media_storage_sop_class_uid: get_element(&data_set, Tag::new(0x02, 0x02))?,
        media_storage_sop_instance_uid: get_element(&data_set, Tag::new(0x02, 0x03))?,
        transfer_syntax_uid: get_element(&data_set, Tag::new(0x0002, 0x0010))?,
        implementation_class_uid: get_element(&data_set, Tag::new(0x0002, 0x0012))?,
        end_position,
        data_set,
    };

    Ok(meta)
}

#[cfg(test)]
pub mod tests {
    use super::parse;
    use crate::handler::data_set::DataSetHandler;

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
        let mut handler = DataSetHandler::default();
        match parse(&mut handler, &bytes) {
            Ok(meta) => {
                assert_eq!(meta.data_set.attributes.len(), 6);
            }
            Err(_parse_error) => {
                assert!(false);
            }
        };
    }
}
