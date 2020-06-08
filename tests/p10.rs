#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Read;
    use dicomparser::accumulator::Accumulator;
    
    use dicomparser::parser::Parser;
    use dicomparser::condition;
    use dicomparser::meta_information;

    #[allow(dead_code)]
    pub fn read_file(filepath: &str) -> Vec<u8> {
        let mut file = File::open(filepath).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn explicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC");
        let accumulator = Accumulator::new(condition::none, condition::none);
        let mut parser = Parser::new(accumulator);
        parser.parse(&mut bytes[132..]);
        println!("{:?}", parser.callback.attributes);

    }

    #[test]
    fn p10_header() {
        let bytes = read_file("tests/fixtures/CT1_UNC");
        let attrs = meta_information::parse(&bytes).unwrap();
        println!("{:?}", attrs);
    }

}