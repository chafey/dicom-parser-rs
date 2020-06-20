#[cfg(test)]
pub mod tests {

    use crate::attribute::Attribute;
    use crate::handler::{Handler, HandlerResult};
    use crate::meta_information;
    use crate::meta_information::MetaInformation;
    use std::fs::File;
    use std::io::Read;

    /// Implementation of Handler that collects the attributes and data from
    /// callback functions and has an optional ability to print the output
    /// of the parse.  This is used by tests and for general debugging

    #[derive(Default)]
    pub struct TestHandler {
        pub attributes: Vec<Attribute>,
        pub data: Vec<Vec<u8>>,
        pub depth: usize,
        pub print: bool,
    }

    impl Handler for TestHandler {
        fn attribute(
            &mut self,
            attribute: &Attribute,
            position: usize,
            data_offset: usize,
        ) -> HandlerResult {
            if self.print {
                println!(
                    "{:-<width$}{:?} (position={}, data_offset={})",
                    "-",
                    attribute,
                    position,
                    data_offset,
                    width = (self.depth * 2)
                );
            }
            self.attributes.push(*attribute);
            HandlerResult::Continue
        }

        fn data(&mut self, _attribute: &Attribute, data: &[u8], complete: bool) {
            if self.print {
                println!(
                    "{:-<width$}+ data of len {:?} complete={}",
                    "-",
                    data.len(),
                    complete,
                    width = (self.depth * 2)
                );
            }
            self.data.push(data.to_vec());
        }

        fn start_sequence(&mut self, _attribute: &Attribute) {
            if self.print {
                println!("{:-<width$}[", "-", width = (self.depth * 2));
            }
            self.depth += 1;
        }

        fn start_sequence_item(&mut self, _attribute: &Attribute) {
            if self.print {
                println!("{:-<width$}{{", "-", width = (self.depth * 2));
            }
            self.depth += 1;
        }

        fn end_sequence_item(&mut self, _attribute: &Attribute) {
            self.depth -= 1;
            if self.print {
                println!("{:-<width$}}}", "-", width = (self.depth * 2));
            }
        }

        fn end_sequence(&mut self, _attribute: &Attribute) {
            self.depth -= 1;
            if self.print {
                println!("{: <width$}]", "-", width = (self.depth * 2));
            }
        }

        fn basic_offset_table(
            &mut self,
            _attribute: &Attribute,
            data: &[u8],
            complete: bool,
        ) -> HandlerResult {
            if self.print {
                println!(
                    "{:-<width$}  \\ basic offset table of len {:?} complete={}",
                    "-",
                    data.len(),
                    complete,
                    width = (self.depth * 2)
                );
            }
            HandlerResult::Continue
        }

        fn pixel_data_fragment(
            &mut self,
            _attribute: &Attribute,
            fragment_number: usize,
            data: &[u8],
            complete: bool,
        ) -> HandlerResult {
            if self.print {
                println!(
                    "{:-<width$}  \\ pixel data fragment #{} data of len {:?} complete={}",
                    "-",
                    fragment_number,
                    data.len(),
                    complete,
                    width = (self.depth * 2)
                );
            }
            self.data.push(data.to_vec());
            HandlerResult::Continue
        }
    }

    pub fn read_file(filepath: &str) -> Vec<u8> {
        let mut file = File::open(filepath).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    pub fn read_data_set_bytes_from_file(filepath: &str) -> (MetaInformation, Vec<u8>) {
        let bytes = read_file(&filepath);
        let mut handler = TestHandler::default();

        let meta = match meta_information::parse(&mut handler, &bytes) {
            Ok(meta) => meta,
            Err(_parse_error) => panic!("Let's play Global Thermonuclear War"),
        };
        //println!("meta.end_position={}", meta.end_position);
        let end_position = meta.end_position;
        (meta, (&bytes[end_position..]).to_vec())
    }
}
