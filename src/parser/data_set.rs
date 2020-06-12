use crate::encoding::Encoding;
use crate::parser::attribute::AttributeParser;
use crate::handler::Handler;
use crate::parser::ParseError;
use crate::parser::ParseState;
use crate::parser::Parser;
use std::marker::PhantomData;

pub struct DataSetParser<'t, T: Encoding> {
    handler: &'t mut dyn Handler,
    parser: Option<Box<dyn Parser<T>>>,
    total_bytes_consumed: usize,
}

impl<'t, T: 'static + Encoding> DataSetParser<'t, T> {
    pub fn new(handler: &'t mut dyn Handler) -> DataSetParser<'t, T> {
        let parser = Box::new(AttributeParser::<T> {
            phantom: PhantomData,
        });

        DataSetParser {
            handler,
            parser: Some(parser),
            total_bytes_consumed: 0,
        }
    }

    pub fn parse(&mut self, bytes: &[u8]) -> Result<(ParseState, usize), ()> {
        let mut remaining_bytes = bytes;
        let mut bytes_consumed = 0;

        while !remaining_bytes.is_empty() {
            match &mut self.parser {
                Some(parser) => {
                    let result = parser.parse(self.handler, remaining_bytes)?;
                    bytes_consumed += result.bytes_consumed;
                    self.total_bytes_consumed += result.bytes_consumed;
                    remaining_bytes = &remaining_bytes[result.bytes_consumed..];
                    match result.state {
                        ParseState::Cancelled => {
                            self.parser = None;
                            return Ok((ParseState::Cancelled, bytes_consumed));
                        }
                        ParseState::Incomplete => {
                            return Ok((ParseState::Incomplete, bytes_consumed));
                        }
                        ParseState::Partial => {
                            self.parser = result.parser;
                        }
                        ParseState::Completed => {
                            self.parser = None;
                            return Ok((ParseState::Completed, bytes_consumed));
                        }
                    }
                }
                None => return Err(()), // parsing cannot continue - either cancelled or completed
            };
        }
        Ok((ParseState::Completed, bytes.len()))
    }
}

// returns
//  number of bytes parsed
//  true if cancelled, false, if not
pub fn parse_full<T: 'static + Encoding>(
    handler: &mut dyn Handler,
    bytes: &[u8],
) -> Result<(usize, bool), ParseError> {
    let mut parser = DataSetParser::<T>::new(handler);
    match parser.parse(bytes) {
        Ok((parse_state, bytes_consumed)) => match parse_state {
            ParseState::Cancelled => Ok((bytes_consumed, true)),
            ParseState::Incomplete => Err(ParseError {}),
            ParseState::Partial => Err(ParseError {}),
            ParseState::Completed => Ok((bytes_consumed, false)),
        },
        Err(()) => Err(ParseError {}),
    }
}

#[cfg(test)]
mod tests {

    use super::DataSetParser;
    use crate::parser::ParseState;
    //use crate::parser::ParseResult;
    use crate::test::tests::read_data_set_bytes_from_file;
    //use crate::parser::attribute::AttributeParser;
    use crate::encoding::ExplicitLittleEndian;
    use crate::handler::data_set::DataSetHandler;
    //use std::marker::PhantomData;

    fn parse_ele_data_set(bytes: &[u8]) -> Result<(ParseState, usize), ()> {
        let mut handler = DataSetHandler::default();
        let mut parser = DataSetParser::<ExplicitLittleEndian>::new(&mut handler);
        parser.parse(bytes)
    }
    #[test]
    fn parse_full_ok() {
        let bytes =
            read_data_set_bytes_from_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let result = parse_ele_data_set(&bytes[..]);
        assert!(result.is_ok());
    }

    /*
        fn split_parse(bytes: &[u8], split_position: usize) -> Result<(), ()> {
            let parser = Box::new(AttributeParser::<ExplicitLittleEndian> {
                phantom: PhantomData,
            });
            let mut handler = DataSetHandler::default();
            //handler.print = true;
            let result = parse_partial::<ExplicitLittleEndian>(&mut handler, &bytes[0..split_position], parser)?;
            println!("bytes_consumed: {:?}", result.bytes_consumed);
            println!("state: {:?}", result.state);
            let result2 = parse_partial::<ExplicitLittleEndian>(&mut handler, &bytes[result.bytes_consumed..], result.parser)?;
            println!("bytes_consumed: {:?}", result2.bytes_consumed);
            println!("state: {:?}", result2.state);
            assert_eq!(result2.bytes_consumed, bytes.len() - result.bytes_consumed);
            Ok(())
        }
    */

    /*
    #[test]
    fn parse_partial_ok() {
        let bytes = read_data_set_bytes_from_file("tests/fixtures/CT1_UNC.explicit_little_endian.dcm");
        let result = split_parse(&bytes, 200000);
        println!("{:?}", result);
    }*/
}
