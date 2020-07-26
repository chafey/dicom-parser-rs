use dicomparser::attribute::Attribute;
use dicomparser::data_set_parser::DataSetParser;
use dicomparser::encoding::ExplicitLittleEndian;
use dicomparser::handler::{Handler, HandlerResult};
use dicomparser::meta_information;
use std::env;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, Error};

#[derive(Default)]
pub struct TestHandler {
    pub depth: usize,
}

impl Handler for TestHandler {
    fn attribute(
        &mut self,
        attribute: &Attribute,
        _position: usize,
        _data_offset: usize,
    ) -> HandlerResult {
        println!("{:-<width$}{:?} ", " ", attribute, width = (self.depth * 2));
        HandlerResult::Continue
    }

    fn data(&mut self, _attribute: &Attribute, data: &[u8], _complete: bool) {
        println!(
            "{:-<width$} data of length {:?}",
            " ",
            data.len(),
            width = (self.depth * 2)
        );
    }

    fn start_sequence(&mut self, _attribute: &Attribute) {
        self.depth += 1;
    }

    fn start_sequence_item(&mut self, _attribute: &Attribute) {
        self.depth += 1;
    }

    fn end_sequence_item(&mut self, _attribute: &Attribute) {
        self.depth -= 1;
    }

    fn end_sequence(&mut self, _attribute: &Attribute) {
        self.depth -= 1;
    }

    fn basic_offset_table(
        &mut self,
        _attribute: &Attribute,
        _data: &[u8],
        _complete: bool,
    ) -> HandlerResult {
        println!("BasicOffsetTable");
        HandlerResult::Continue
    }

    fn pixel_data_fragment(
        &mut self,
        _attribute: &Attribute,
        _fragment_number: usize,
        data: &[u8],
        _complete: bool,
    ) -> HandlerResult {
        println!("Pixle Data Fragment of length {:?} ", data.len());
        HandlerResult::Continue
    }
}

async fn reader(file_path: &str) -> Result<(), Error> {
    let mut file = File::open(file_path).await?;
    let mut bytes = vec![0; 1024];

    let mut handler = TestHandler::default();
    // Read the first 1k bytes which is hopefully enough for
    // the p10 header (ugh)
    let num_bytes_read = file.read(&mut bytes).await?;
    let meta = match meta_information::parse(&mut handler, &bytes[..num_bytes_read]) {
        Ok(meta) => meta,
        Err(_err) => return Ok(()),
    };
    println!("{:?}", meta);
    let mut parser = DataSetParser::<ExplicitLittleEndian>::default();

    let mut remaining_bytes = (&bytes[meta.end_position..num_bytes_read]).to_vec();
    let mut bytes_from_beginning = meta.end_position;
    //let result = parser.parse(&mut handler, &remaining_bytes, bytes_from_beginning);
    let mut content = vec![0; 1024 * 64];
    loop {
        //println!("remaining_bytes.len()={:?}", remaining_bytes.len());
        let num_bytes_read2 = file.read(&mut content).await?;
        //println!("read {:?} bytes", num_bytes_read2);
        if num_bytes_read2 == 0 {
            break;
        }

        // append to remaining bytes
        let concat = [&remaining_bytes, &content[..num_bytes_read2]].concat();

        //println!("concat.len()={:?}", concat.len());

        match parser.parse(&mut handler, &concat, bytes_from_beginning) {
            Ok(result) => {
                //println!("bytes consumed={:?}", result.bytes_consumed);
                bytes_from_beginning += result.bytes_consumed;
                //println!("bytes from beginning: {}", bytes_from_beginning);
                remaining_bytes = (&concat[result.bytes_consumed..]).to_vec();
            }
            Err(_) => return Ok(()),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    reader(&args[1]).await.unwrap();
}
