use crate::attribute::Attribute;
use crate::data_set::DataSet;
use crate::data_set_parser::parse_full;
use crate::encoding::ExplicitLittleEndian;
use crate::handler::cancel::CancelHandler;
use crate::handler::data_set::DataSetHandler;
use crate::prefix;
use crate::tag::Tag;
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
fn find_element_index(attributes: &[Attribute], tag: Tag) -> Result<usize, ()> {
    for (index, attribute) in attributes.iter().enumerate() {
        if attribute.tag == tag {
            return Ok(index);
        }
    }
    Err(())
}

/// Helper function to return the data of an element in a DataSet as a UTF8 string.
fn get_element(dataset: &DataSet, tag: Tag) -> Result<String, ()> {
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
pub fn parse(bytes: &[u8]) -> Result<MetaInformation, ()> {
    if !prefix::detect(bytes) {
        return Err(());
    }

    let mut data_set_handler = DataSetHandler::default();

    let mut handler = CancelHandler::new(&mut data_set_handler, |x: &Attribute| x.tag.group != 2);

    let end_position = match parse_full::<ExplicitLittleEndian>(&mut handler, &bytes[132..], 132) {
        Ok((bytes_consumed, _cancelled)) => {
            // note, we expect to be cancelled, but don't check for it as it is possible
            // that the caller is only passing in the header
            bytes_consumed + 132
        }
        Err(_parse_error) => return Err(()),
    };

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
        let meta = parse(&bytes).unwrap();
        assert_eq!(meta.data_set.attributes.len(), 6);
    }
}
