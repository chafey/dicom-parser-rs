use crate::callback::{Callback};
use crate::attribute::Attribute;

pub struct Parser<T:Callback> {
    callback: T,
    buffer: Vec<u8>,
    buffer_position: usize, // read position in current buffer
    data_position: usize, // position from first byte parsed
    element_data_bytes_remaining: usize    
}

impl<T: crate::callback::Callback> Parser<T> {
    pub fn new(callback: T) -> Parser<T> {
        Parser {
            callback: callback,
            buffer: vec![],
            buffer_position: 0,
            data_position: 0,
            element_data_bytes_remaining: 0
        }
    }

    pub fn parse(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(&bytes);

        loop {
            if self.element_data_bytes_remaining > 0{
                if (self.buffer.len() + self.buffer_position) > self.element_data_bytes_remaining {
                    self.callback.data(&self.buffer[self.buffer_position..self.buffer_position+self.element_data_bytes_remaining]);
                    self.buffer_position += self.element_data_bytes_remaining;
                    self.element_data_bytes_remaining = 0;
                } else {
                    return;
                }
            }
    
            if (self.buffer.len() - self.buffer_position) >= 10 {
                let mut attr = Attribute::ele(&self.buffer);
                self.buffer_position += attr.data_position;
                self.data_position += attr.data_position;
                attr.data_position = self.data_position;
                self.element_data_bytes_remaining = attr.length;
                self.callback.element(attr);
            } else {
                return;
            }
        } 
    }
}

#[cfg(test)]
mod tests {
    use crate::callback::Callback;
    use crate::attribute::Attribute;
    use super::Parser;

    struct TestCallback {
        pub attributes: Vec<Attribute> 
    }

    impl Callback for TestCallback {
        fn element(&mut self, attribute: Attribute) {
            println!("{:?}", attribute);
            self.attributes.push(attribute);
        }

        fn data(&mut self, data: &[u8]) {
            println!("data of len {:?}", data.len())
        }
    }


    #[test]
    fn can_parse() {
        let callback = TestCallback{attributes : vec![]};
        let mut parser = Parser::<TestCallback>::new(callback);
        let bytes = vec![8,0, 8,0, 0x43,0x53, 2,0, 0,0, 1, 2];
        parser.parse(&bytes);
        assert_eq!(parser.callback.attributes.len(), 1);
        assert_eq!(parser.callback.attributes[0].group, 8);
        assert_eq!(parser.callback.attributes[0].element, 8);
        assert_eq!(parser.callback.attributes[0].vr[0], b'C');
        assert_eq!(parser.callback.attributes[0].vr[1], b'S');
        assert_eq!(parser.callback.attributes[0].length, 2);
    }

}