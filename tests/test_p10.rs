#[cfg(test)]
mod tests {

    use dicomparser::accumulator::Accumulator;
    use std::fs::File;
    use std::io::Read;

    use dicomparser::condition;
    use dicomparser::p10;

    #[allow(dead_code)]
    pub fn read_file(filepath: &str) -> Vec<u8> {
        let mut file = File::open(filepath).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn explicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let mut accumulator = Accumulator::new(condition::none, condition::none);
        p10::parse(&mut accumulator, &mut bytes).unwrap();
        println!("Parsed {:?} attributes", accumulator.attributes.len());
        //println!("{:?}", accumulator.attributes);
    }

    #[test]
    fn implicit_little_endian() {
        let mut bytes = read_file("tests/fixtures/CT1_UNC.implicit_little_endian.dcm");
        let mut accumulator = Accumulator::new(condition::none, condition::none);
        p10::parse(&mut accumulator, &mut bytes).unwrap();
        println!("Parsed {:?} attributes", accumulator.attributes.len());
        //println!("{:?}", accumulator.attributes);
    }


    #[test]
    fn sequences() {
        //(0008,9121) @ position 0x376 / 886
        let mut bytes = read_file("tests/fixtures/CT0012.fragmented_no_bot_jpeg_ls.80.dcm");
        let mut accumulator = Accumulator::new(condition::none, condition::none);
        p10::parse(&mut accumulator, &mut bytes).unwrap();
        println!("Parsed {:?} attributes", accumulator.attributes.len());
        /*for attr in accumulator.attributes {
            println!("{:?}", attr);
        }*/
    }

    /*
        #[test]
        fn explicit_big_endian() {
            let mut bytes = read_file("tests/fixtures/CT1_UNC.explicit_big_endian.dcm");
            let meta = meta_information::parse(&bytes).unwrap();
            let accumulator = Accumulator::new(condition::none, condition::none);
            let mut parser = Parser::new(accumulator, Attribute::ele);
            parser.parse(&mut bytes[meta.end_position..]);
            println!("Parsed {:?} attributes", parser.callback.attributes.len());
        }
    */
}
