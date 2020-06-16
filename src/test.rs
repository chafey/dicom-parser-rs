#[cfg(test)]
pub mod tests {

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
        let meta = meta_information::parse(&bytes).unwrap();
        //println!("meta.end_position={}", meta.end_position);
        let end_position = meta.end_position;
        (meta, (&bytes[end_position..]).to_vec())
    }
}
