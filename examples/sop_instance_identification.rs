use dicomparser::attribute::Attribute;
use dicomparser::handler::{Handler, HandlerResult};
use dicomparser::p10::parse;
use dicomparser::tag::Tag;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

static STUDYINSTANCEUID: Tag = Tag {
    group: 0x0020,
    element: 0x000D,
};

static SERIESINSTANCEUID: Tag = Tag {
    group: 0x0020,
    element: 0x000E,
};

static SOPINSTANCEUID: Tag = Tag {
    group: 0x0008,
    element: 0x0018,
};

#[derive(Default, Debug)]
pub struct SOPInstanceIdentificationHandler {
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub sop_instance_uid: String,
    // buffer to accumulate data for an attribute
    data_buffer: Vec<u8>,
}

impl SOPInstanceIdentificationHandler {
    fn is_tag_wanted(tag: Tag) -> bool {
        if tag == STUDYINSTANCEUID {
            true
        } else if tag == SERIESINSTANCEUID {
            true
        } else if tag == SOPINSTANCEUID {
            true
        } else {
            false
        }
    }
}

impl Handler for SOPInstanceIdentificationHandler {
    fn attribute(
        &mut self,
        attribute: &Attribute,
        _position: usize,
        _data_offset: usize,
    ) -> HandlerResult {
        if SOPInstanceIdentificationHandler::is_tag_wanted(attribute.tag) {
            self.data_buffer.clear();
        }
        if attribute.tag > Tag::new(0x0020, 0x000E) {
            HandlerResult::Cancel
        } else {
            HandlerResult::Continue
        }
    }

    fn data(&mut self, attribute: &Attribute, data: &[u8], complete: bool) {
        if attribute.length == 0 {
            return;
        }
        if SOPInstanceIdentificationHandler::is_tag_wanted(attribute.tag) {
            self.data_buffer.extend_from_slice(data);

            if complete {
                let bytes = if self.data_buffer[attribute.length - 1] != 0 {
                    &self.data_buffer
                } else {
                    &self.data_buffer[0..(attribute.length - 1)]
                };

                if attribute.tag == STUDYINSTANCEUID {
                    self.study_instance_uid = String::from(str::from_utf8(&bytes).unwrap());
                } else if attribute.tag == SERIESINSTANCEUID {
                    self.series_instance_uid = String::from(str::from_utf8(&bytes).unwrap());
                } else if attribute.tag == SOPINSTANCEUID {
                    self.sop_instance_uid = String::from(str::from_utf8(&bytes).unwrap());
                }
            }
        }
    }
}

pub fn read_file(filepath: &str) -> Vec<u8> {
    let mut file = File::open(filepath).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bytes = if args.len() > 1 {
        read_file(&args[1])
    } else {
        read_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm")
    };
    let mut handler = SOPInstanceIdentificationHandler::default();
    match parse(&mut handler, &mut bytes) {
        Ok(_meta) => println!("{:?}", handler),
        Err(_parse_error) => {}
    }
}
