use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;

use std::fmt;

pub struct ParseError {
    pub reason: &'static str,
    pub position: usize,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ParseState {
    Cancelled,  // parse was cancelled by Handler
    Incomplete, // attribute data not fully parsed due to lack of bytes
    Completed,  // attribute data fully parsed
}

pub struct ParseResult {
    pub bytes_consumed: usize,
    pub state: ParseState,
}

impl ParseResult {
    pub fn cancelled(bytes_consumed: usize) -> ParseResult {
        ParseResult {
            bytes_consumed,
            state: ParseState::Cancelled,
        }
    }

    pub fn incomplete(bytes_consumed: usize) -> ParseResult {
        ParseResult {
            bytes_consumed,
            state: ParseState::Incomplete,
        }
    }

    pub fn completed(bytes_consumed: usize) -> ParseResult {
        ParseResult {
            bytes_consumed,
            state: ParseState::Completed,
        }
    }
}

//
// This trait defines an interface for parsing the data portion of a DICOM Attribute
//
pub trait Parser<T: Encoding + fmt::Debug> {
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        position: usize,
    ) -> Result<ParseResult, ParseError>;
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
