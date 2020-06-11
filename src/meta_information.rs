use crate::attribute::Attribute;
use crate::data_set::DataSet;
use crate::encoding::ExplicitLittleEndian;
use crate::handler::data_set::DataSetHandler;
use crate::handler::stop::StopHandler;
use crate::parser::data_set;
use crate::prefix;
use crate::tag::Tag;
use std::str;

#[derive(Debug)]
pub struct MetaInformation {
    pub media_storage_sop_class_uid: String,
    pub media_storage_sop_instance_uid: String,
    pub transfer_syntax_uid: String,
    pub implementation_class_uid: String,
    pub end_position: usize,
    pub data_set: DataSet,
}

fn find_element_index(attributes: &[Attribute], tag: Tag) -> Result<usize, ()> {
    for (index, attribute) in attributes.iter().enumerate() {
        if attribute.tag == tag {
            return Ok(index);
        }
    }
    Err(())
}

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

pub fn parse(bytes: &[u8]) -> Result<MetaInformation, ()> {
    if !prefix::detect(bytes) {
        return Err(());
    }

    let mut data_set_handler = DataSetHandler::default();

    let mut handler = StopHandler {
        stop_fn: |x: &Attribute| x.tag.group != 2,
        handler: &mut data_set_handler,
    };

    let end_position =
        match data_set::parse_full::<ExplicitLittleEndian>(&mut handler, &bytes[132..]) {
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
