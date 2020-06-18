use crate::attribute::Attribute;
use crate::encoding::Encoding;
use crate::handler::Handler;

use std::fmt;

/// Contains information about an error encountered while parsing
pub struct ParseError {
    /// A string explaining the reason for the parse failure
    pub reason: &'static str,
    /// The position relative to the beginning of the stream that the error
    /// occured at
    pub position: usize,
}

/// Enum describing the current state of the parser
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ParseState {
    /// The parse was canceled by the Handler
    Cancelled,
    /// The parse is incomplete due to lack of bytes provided.  Parsing
    /// can continue once more bytes are available
    Incomplete,
    /// The Attribute and its value field were fully parsed
    Completed,
}

/// Contains information about the result of a parse
pub struct ParseResult {
    /// The number of bytes actually consumed by this function
    pub bytes_consumed: usize,
    /// The result state of the parser after calling this function
    pub state: ParseState,
}

impl ParseResult {
    /// Convenience function to create a Cancelled ParseResult
    pub fn cancelled(bytes_consumed: usize) -> ParseResult {
        ParseResult {
            bytes_consumed,
            state: ParseState::Cancelled,
        }
    }

    /// Convenience function to create an Incomplete ParseResult
    pub fn incomplete(bytes_consumed: usize) -> ParseResult {
        ParseResult {
            bytes_consumed,
            state: ParseState::Incomplete,
        }
    }

    /// Convenience function to create an Completed ParseResult
    pub fn completed(bytes_consumed: usize) -> ParseResult {
        ParseResult {
            bytes_consumed,
            state: ParseState::Completed,
        }
    }
}

/// This trait defines an interface for parsing the value portion of a DICOM
/// Attribute for a specific Encoding.
pub trait ValueParser<T: Encoding + fmt::Debug> {
    /// Parses the value field
    ///
    /// # Arguments
    ///
    /// * `handler`   - The Handler to invoke when parsing the value field
    /// * `attribute` - The Attribute associated with this value field
    /// * `bytes`     - The raw bytes of the value field
    /// * `position`  - The position since the beginning of the parse stream
    ///                 of the value field.
    fn parse(
        &mut self,
        handler: &mut dyn Handler,
        attribute: &Attribute,
        bytes: &[u8],
        position: usize,
    ) -> Result<ParseResult, ParseError>;
}

pub mod basic_offset_table;
pub mod data;
pub mod data_undefined_length;
pub mod encapsulated_pixel_data;
pub mod pixel_data_fragment;
pub mod sequence;
pub mod sequence_item_data;
