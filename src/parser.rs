use crate::attribute::Attribute;

#[derive(PartialEq)]
pub enum Control {
    Element, // skip data
    Data, // send data
    Stop, // stop parsing
}

pub trait Callback
{
    fn element(&mut self, attribute: Attribute) -> Control;
    fn data(&mut self, data: &[u8]);
}

pub struct Parser<T:Callback> {
    pub callback: T,
    buffer: Vec<u8>,
    buffer_position: usize, // read position in current buffer
    data_position: usize, // position from first byte parsed
    element_data_bytes_remaining: usize,
    state: Control    
}

impl<T: Callback> Parser<T> {
    pub fn new(callback: T) -> Parser<T> {
        Parser {
            callback,
            buffer: vec![],
            buffer_position: 0,
            data_position: 0,
            element_data_bytes_remaining: 0,
            state: Control::Element
        }
    }

    pub fn parse(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(&bytes);

        while self.state != Control::Stop {
            if self.element_data_bytes_remaining > 0{
                if (self.buffer.len() - self.buffer_position) >= self.element_data_bytes_remaining {
                    if self.state == Control::Data {
                        self.callback.data(&self.buffer[self.buffer_position..self.buffer_position+self.element_data_bytes_remaining]);
                        self.state = Control::Element;
                    }
                    
                    self.buffer_position += self.element_data_bytes_remaining;
                    self.data_position += self.element_data_bytes_remaining;
                    self.element_data_bytes_remaining = 0;
                } else {
                    return;
                }
            }
    
            if (self.buffer.len() - self.buffer_position) >= 10 {
                let mut attr = Attribute::ele(&self.buffer[self.buffer_position..]);
                self.buffer_position += attr.data_position;
                self.data_position += attr.data_position;
                attr.data_position = self.data_position;
                self.element_data_bytes_remaining = attr.length;
                self.state = self.callback.element(attr);
            } else {
                return;
            }
        } 
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::Attribute;
    use crate::tag::Tag;
    use super::{Parser, Callback, Control};

    struct TestCallback {
        pub attributes: Vec<Attribute>,
        pub data: Vec<Vec<u8>>
    }

    impl Callback for TestCallback {
        fn element(&mut self, attribute: Attribute) -> Control {
            //println!("{:?}", attribute);
            self.attributes.push(attribute);
            Control::Data
        }

        fn data(&mut self, data: &[u8]) {
            //println!("data of len {:?}", data.len());
            self.data.push(data.to_vec());
        }
    }

    #[test]
    fn can_parse() {
        let callback = TestCallback{
            attributes : vec![],
            data: vec![]
        };
        let mut parser = Parser::<TestCallback>::new(callback);
        let bytes = vec![8,0, 8,0, 0x43,0x53, 4,0, 1,2,3,4,
        8,0, 8,0, 0x43,0x53, 2,0, 0,0];
        parser.parse(&bytes);
        assert_eq!(parser.callback.attributes.len(), 2);
        assert_eq!(parser.callback.attributes[0].tag, Tag::new(8,8));
        assert_eq!(parser.callback.attributes[0].vr[0], b'C');
        assert_eq!(parser.callback.attributes[0].vr[1], b'S');
        assert_eq!(parser.callback.attributes[0].length, 4);
        assert_eq!(parser.callback.attributes[1].tag, Tag::new(8,8));
        assert_eq!(parser.callback.attributes[1].vr[0], b'C');
        assert_eq!(parser.callback.attributes[1].vr[1], b'S');
        assert_eq!(parser.callback.attributes[1].length, 2);
        assert_eq!(parser.callback.data.len(), 2);
        assert_eq!(parser.callback.data[0].len(), 4);
        assert_eq!(parser.callback.data[0][0], 1);
        assert_eq!(parser.callback.data[0][1], 2);
        assert_eq!(parser.callback.data[0][2], 3);
        assert_eq!(parser.callback.data[0][3], 4);
        assert_eq!(parser.callback.data[1].len(), 2);
        assert_eq!(parser.callback.data[1][0], 0);
        assert_eq!(parser.callback.data[1][1], 0);

    }

}