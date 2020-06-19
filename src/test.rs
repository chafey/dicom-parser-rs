#[cfg(test)]
pub mod tests {

    use crate::handler::data_set::DataSetHandler;
    use crate::meta_information;
    use crate::meta_information::MetaInformation;
    use std::fs::File;
    use std::io::Read;

    pub fn read_file(filepath: &str) -> Vec<u8> {
        let mut file = File::open(filepath).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    pub fn read_data_set_bytes_from_file(filepath: &str) -> (MetaInformation, Vec<u8>) {
        let bytes = read_file(&filepath);
        let mut handler = DataSetHandler::default();

        let meta = match meta_information::parse(&mut handler, &bytes) {
            Ok(meta) => meta,
            Err(_parse_error) => panic!("Let's play Global Thermonuclear War"),
        };
        //println!("meta.end_position={}", meta.end_position);
        let end_position = meta.end_position;
        (meta, (&bytes[end_position..]).to_vec())
    }
}
