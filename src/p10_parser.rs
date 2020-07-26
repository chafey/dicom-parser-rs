/*use crate::data_set_parser::DataSetParser;
use crate::encoding::Encoding;
use crate::encoding::ImplicitLittleEndian;
use crate::handler::Handler;
use crate::meta_information;
use crate::meta_information::MetaInformation;
use crate::value_parser::ParseError;
use crate::value_parser::ParseResult;

#[derive(Debug, Default)]
pub struct P10Parser {
    bytes_consumed: usize,
    meta: Option<MetaInformation>,
    parser: Option<Box<DataSetParser<dyn Encoding>>>,
}

impl P10Parser {
    pub fn parse<'a, T: Handler>(
        &mut self,
        handler: &'a mut T,
        bytes: &[u8],
    ) -> Result<ParseResult, ParseError> {
        match self.meta {
            None => {
                let meta = meta_information::parse(handler, bytes)?;
                self.meta = Some(meta);
                self.bytes_consumed = meta.end_position;
                self.parser = match &meta.transfer_syntax_uid[..] {
                    "1.2.840.10008.1.2" => {
                        Some(Box::new(DataSetParser::<ImplicitLittleEndian>::default()))
                    }
                }
            }
            Some(_) => {}
        }

        println!("{:?}", self.meta);

        Err(ParseError {
            reason: "unexpected EOF",
            position: self.bytes_consumed,
        })
    }
}
#[cfg(test)]
mod tests {

    use super::P10Parser;
    use crate::test::tests::read_file;
    use crate::test::tests::TestHandler;

    #[test]
    fn explicit_little_endian() {
        let bytes = read_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let mut handler = TestHandler::default();
        //handler.print = true;
        let mut parser = P10Parser::default();
        let result = parser.parse(&mut handler, &bytes);
        assert!(result.is_ok());
        assert_eq!(265, handler.attributes.len());
    }
}
*/
git