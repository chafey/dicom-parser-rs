use crate::encoding::Encoding;
use crate::handler::Handler;
use std::fmt;

pub struct ParseError {}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ParseState {
    Cancelled,  // parse was cancelled by client
    Incomplete, // cannot parse due to lack of bytes
    Completed,  // parse completed for this attribute/element
}

pub struct ParseResult<T: Encoding> {
    pub bytes_consumed: usize,
    pub parser: Option<Box<dyn Parser<T>>>,
    pub state: ParseState,
}

impl<T: Encoding> ParseResult<T> {
    pub fn cancelled(bytes_consumed: usize) -> ParseResult<T> {
        ParseResult::<T> {
            bytes_consumed,
            parser: None,
            state: ParseState::Cancelled,
        }
    }

    pub fn incomplete(bytes_consumed: usize) -> ParseResult<T> {
        ParseResult::<T> {
            bytes_consumed,
            parser: None,
            state: ParseState::Incomplete,
        }
    }

    pub fn completed(bytes_consumed: usize) -> ParseResult<T> {
        ParseResult::<T> {
            bytes_consumed,
            parser: None,
            state: ParseState::Completed,
        }
    }
}

//
// This trait defines an interface for parsing a portion of a DICOM bitstream
//
pub trait Parser<T: Encoding + fmt::Debug> {
    // parses bytes.  possible outcomes
    //  - parse manually cancelled (stopped) - parsing should not continued
    //  - parse manually suspended - parsing can continue if desired.  *Requires state management
    //  - parse pending - due to lack of bytes to complete parse - this is expected while streaming.  *Requires state management
    //  - parse completed - all bytes provided parsed - this can occur while streaming and does not indicate end of the parse
    //  - unrecoverable error - parsing cannot continue
    fn parse(&mut self, handler: &mut dyn Handler, bytes: &[u8]) -> Result<ParseResult<T>, ()>;
}

pub mod attribute;
pub mod basic_offset_table;
pub mod data;
pub mod data_set;
pub mod data_undefined_length;
pub mod encapsulated_pixel_data;
pub mod pixel_data_fragment;
pub mod sequence;
pub mod sequence_item_data;
