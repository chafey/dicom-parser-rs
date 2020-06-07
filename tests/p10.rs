#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Read;
    use dicomparser::accumulator::Accumulator;
    
    use dicomparser::parser::Parser;

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
        let accumulator = Accumulator::new();
        let mut parser = Parser::new(accumulator);
        parser.parse(&mut bytes[132..]);
    }
}