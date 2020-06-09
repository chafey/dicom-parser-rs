use crate::attribute::{Attribute, AttributeFN};
use crate::sequence_item;
use crate::vr::VR;

#[derive(PartialEq)]
pub enum Control {
    Element, // skip to next element
    Data,    // decode element data
    Stop,    // stop parsing
}

pub trait Callback {
    fn element(&mut self, attribute: &Attribute) -> Control;
    fn data(&mut self, attribute: &Attribute, data: &[u8]);
    fn start_sequence_item(&mut self, attribute: &Attribute);
    fn end_sequence_item(&mut self, attribute: &Attribute);
}

struct SequenceItem {
    pub attribute: Attribute,
    pub item_end_position: usize,
}

pub struct Parser<'a, T>
where
    T: 'a,
{
    pub callback: &'a mut T,
    attribute_fn: AttributeFN,
    buffer: Vec<u8>,
    buffer_position: usize, // read position in current buffer
    data_position: usize,   // position from first byte parsed
    element_data_bytes_remaining: usize,
    state: Control,
    attribute: Option<Attribute>,
    sequence_items: Vec<SequenceItem>,
}

impl<'a, T: Callback> Parser<'a, T> {
    pub fn new(callback: &'a mut T, attribute_fn: AttributeFN) -> Parser<T> {
        Parser {
            callback,
            attribute_fn,
            buffer: vec![],
            buffer_position: 0,
            data_position: 0,
            attribute: None,
            element_data_bytes_remaining: 0,
            state: Control::Element,
            sequence_items: vec![],
        }
    }

    fn handle_element(&mut self) {
        let mut attribute = (self.attribute_fn)(&self.buffer[self.buffer_position..]);
        // TODO: Handle undefined lengths (-1)
        self.buffer_position += attribute.data_position;
        self.data_position += attribute.data_position;
        attribute.data_position = self.data_position;
        self.element_data_bytes_remaining = attribute.length;
        self.state = self.callback.element(&attribute);
        self.attribute = Some(attribute);
    }

    fn handle_data(&mut self) {
        if self.state == Control::Data {
            self.callback.data(
                &self.attribute.unwrap(),
                &self.buffer[self.buffer_position
                    ..self.buffer_position + self.element_data_bytes_remaining],
            );
            self.state = Control::Element;
        }

        self.buffer_position += self.element_data_bytes_remaining;
        self.data_position += self.element_data_bytes_remaining;
        self.element_data_bytes_remaining = 0;
    }

    fn handle_sequence(&mut self) {
        let sequence_item = &(self.buffer[self.buffer_position..self.buffer_position + 8]);
        println!(
            "sequence_item = {:02X},{:02X},{:02X},{:02X}",
            sequence_item[0], sequence_item[1], sequence_item[2], sequence_item[3]
        );
        let sequence_item_length = sequence_item::read(sequence_item).unwrap();
        self.buffer_position += 8;
        self.data_position += 8;
        self.callback.start_sequence_item(&self.attribute.unwrap());
        self.sequence_items.push(SequenceItem {
            attribute: self.attribute.unwrap(),
            item_end_position: self.data_position + sequence_item_length,
        });
    }

    pub fn parse(&mut self, bytes: &[u8]) {
        if self.state == Control::Stop {
            return;
        }

        self.buffer.extend_from_slice(&bytes);

        while self.state != Control::Stop {
            if self.element_data_bytes_remaining > 0 {
                if (self.buffer.len() - self.buffer_position) >= self.element_data_bytes_remaining {
                    if self.attribute.unwrap().vr == Some(VR::SQ) {
                        self.handle_sequence();
                    } else {
                        self.handle_data();
                    }
                } else {
                    return;
                }
            }

            if (self.buffer.len() - self.buffer_position) >= 10 {
                if let Some(sequence_item) = self.sequence_items.last() {
                    if sequence_item.item_end_position == self.data_position {
                        self.callback.end_sequence_item(&sequence_item.attribute);
                        let end_sequence_data =
                            sequence_item.attribute.data_position + sequence_item.attribute.length;
                        if end_sequence_data == (self.data_position) {
                            // end of sequence items
                            //sequence_item::read_item_end(&);
                            //self.buffer_position += 4;
                            //self.data_position += 4;
                            self.sequence_items.pop();
                        } else {
                            // another sequence item
                            self.attribute = Some(sequence_item.attribute);
                            self.sequence_items.pop();
                            self.handle_sequence();
                        }
                        self.state = Control::Data;
                        continue;
                    }
                }
                self.handle_element();
            } else {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Callback, Control, Parser};
    use crate::attribute::Attribute;
    use crate::tag::Tag;
    use crate::vr::VR;

    struct TestCallback {
        pub attributes: Vec<Attribute>,
        pub data: Vec<Vec<u8>>,
    }

    impl Callback for TestCallback {
        fn element(&mut self, attribute: &Attribute) -> Control {
            //println!("{:?}", attribute);
            self.attributes.push(*attribute);
            Control::Data
        }

        fn data(&mut self, _attribute: &Attribute, data: &[u8]) {
            //println!("data of len {:?}", data.len());
            self.data.push(data.to_vec());
        }

        fn start_sequence_item(&mut self, _attribute: &Attribute) {}

        fn end_sequence_item(&mut self, _attribute: &Attribute) {}
    }

    fn make_dataset() -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&vec![0x02, 0x00, 0x00, 0x00, b'U', b'L', 4, 0, 0, 0, 0, 0]);
        bytes.extend_from_slice(&vec![
            0x02, 0x00, 0x01, 0x00, b'O', b'B', 0, 0, 2, 0, 0, 0, 0, 1,
        ]);

        bytes
    }

    #[test]
    fn full_parse() {
        let mut callback = TestCallback {
            attributes: vec![],
            data: vec![],
        };
        let mut parser = Parser::<TestCallback>::new(&mut callback, Attribute::ele);
        let bytes = make_dataset();
        parser.parse(&bytes);
        assert_eq!(parser.callback.attributes.len(), 2);
        assert_eq!(parser.callback.attributes[0].tag, Tag::new(2, 0));
        assert_eq!(parser.callback.attributes[0].vr, Some(VR::UL));
        assert_eq!(parser.callback.attributes[0].length, 4);
        assert_eq!(parser.callback.attributes[1].tag, Tag::new(2, 1));
        assert_eq!(parser.callback.attributes[1].vr, Some(VR::OB));
        assert_eq!(parser.callback.attributes[1].length, 2);
        assert_eq!(parser.callback.data.len(), 2);
        assert_eq!(parser.callback.data[0].len(), 4);
        assert_eq!(parser.callback.data[0][0], 0);
        assert_eq!(parser.callback.data[0][1], 0);
        assert_eq!(parser.callback.data[0][2], 0);
        assert_eq!(parser.callback.data[0][3], 0);
        assert_eq!(parser.callback.data[1].len(), 2);
        assert_eq!(parser.callback.data[1][0], 0);
        assert_eq!(parser.callback.data[1][1], 1);
    }

    #[test]
    fn streaming_parse() {
        let mut callback = TestCallback {
            attributes: vec![],
            data: vec![],
        };
        let mut parser = Parser::<TestCallback>::new(&mut callback, Attribute::ele);
        let bytes = make_dataset();
        parser.parse(&bytes[0..5]);
        parser.parse(&bytes[5..9]);
        parser.parse(&bytes[9..19]);
        parser.parse(&bytes[19..]);
        assert_eq!(parser.callback.attributes.len(), 2);
        assert_eq!(parser.callback.attributes[0].tag, Tag::new(2, 0));
        assert_eq!(parser.callback.attributes[0].vr, Some(VR::UL));
        assert_eq!(parser.callback.attributes[0].length, 4);
        assert_eq!(parser.callback.attributes[1].tag, Tag::new(2, 1));
        assert_eq!(parser.callback.attributes[1].vr, Some(VR::OB));
        assert_eq!(parser.callback.attributes[1].length, 2);
        assert_eq!(parser.callback.data.len(), 2);
        assert_eq!(parser.callback.data[0].len(), 4);
        assert_eq!(parser.callback.data[0][0], 0);
        assert_eq!(parser.callback.data[0][1], 0);
        assert_eq!(parser.callback.data[0][2], 0);
        assert_eq!(parser.callback.data[0][3], 0);
        assert_eq!(parser.callback.data[1].len(), 2);
        assert_eq!(parser.callback.data[1][0], 0);
        assert_eq!(parser.callback.data[1][1], 1);
    }

    struct StopCallback {
        pub element_count: usize,
        pub data_count: usize,
    }

    impl Callback for StopCallback {
        fn element(&mut self, _attribute: &Attribute) -> Control {
            self.element_count += 1;
            Control::Stop
        }

        fn data(&mut self, _attribute: &Attribute, _data: &[u8]) {
            self.data_count += 1;
        }

        fn start_sequence_item(&mut self, _attribute: &Attribute) {}

        fn end_sequence_item(&mut self, _attribute: &Attribute) {}
    }

    #[test]
    fn parse_stops() {
        let mut callback = StopCallback {
            element_count: 0,
            data_count: 0,
        };
        let mut parser = Parser::<StopCallback>::new(&mut callback, Attribute::ele);
        let bytes = make_dataset();
        parser.parse(&bytes);
        assert_eq!(parser.callback.element_count, 1);
        assert_eq!(parser.callback.data_count, 0);
    }
}
