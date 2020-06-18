use crate::attribute::Attribute;

/// Enum returned from Handler implementations to instruct the parser to
/// continue parsing or cancel parsing.  
#[derive(PartialEq)]
pub enum HandlerResult {
    Continue, // continue (decode the element's data)
    Cancel,   // stop parsing
}

/// The Handler trait defines a callback interface that is called when
/// parsing a DICOM DataSet allowing the parsed data to be processed.
/// Note that DICOM DataSet are tree like due to sequences, implementations
/// of this trait must be aware and keep track of this.
pub trait Handler {
    /// Invoked every time an Attribute is parsed.  Note that the data for the
    /// attribute is provided via the data function below and may have not been
    /// provided to the parser yet (due to streaming).  
    ///
    /// # Arguments
    ///
    /// * `_attribute`   - The attribute parsed (Tag, VR, Length)
    /// * `_position`    - The offset from the beginning of the stream of the
    ///                    first byte of the Attribute
    /// * `_data_offset` - The offset from _position of the attribute's value
    ///                    field
    fn attribute(
        &mut self,
        _attribute: &Attribute,
        _position: usize,
        _data_offset: usize,
    ) -> HandlerResult {
        HandlerResult::Continue
    }
    /// Invoked after attribute() with the value field/data for the attribute
    /// This function may be invoked multiple times for the same attribute
    /// due to streaming.  Handler implementations are responsible for
    /// concatenating the received data in this case
    ///
    /// # Arguments
    /// * `_attribute` - the Attribute corresponding to this data
    /// * `_data`      - the raw bytes for the value field
    fn data(&mut self, _attribute: &Attribute, _data: &[u8]) {}
    /// Invoked after attribute() for Sequences Attributes instead of data().
    /// A corresponding call to end_sequence() will be made once the value
    /// field for the sequence is fully parsed.
    fn start_sequence(&mut self, _attribute: &Attribute) {}
    /// Invoked for each sequence item parsed in a sequence attribute.  A
    /// corresponding call to end_sequence_item() will be made once the
    /// sequence item is fully parsed.  Parsing a sequence item includes
    /// zero or more calls to attribute() for each attribute in the sequence
    /// item
    fn start_sequence_item(&mut self, _attribute: &Attribute) {}
    /// Invoked after all attributes in a sequence item are parsed.  
    /// Corresponds to exactly one prior call to start_sequence_item()
    fn end_sequence_item(&mut self, _attribute: &Attribute) {}
    /// Invoked once the value field for a sequence is fully parsed.  
    /// Corresponds to exaclty one prior call to start_sequence
    fn end_sequence(&mut self, _attribute: &Attribute) {}
    /// Invoked when the basic offset table is parsed in an encaspulated pixel
    /// data attribute.  Note that basic offset table is not required so may be
    /// empty (or zero length)
    /// This function may be invoked multiple times for the same attribute
    /// due to streaming.  Handler implementations are responsible for
    /// concatenating the received data in this case
    fn basic_offset_table(&mut self, _attribute: &Attribute, _data: &[u8]) -> HandlerResult {
        HandlerResult::Continue
    }
    /// Invoked for each pixel data fragment parsed in an encapsulated pixel
    /// data attribute.  Note that a given image frame may consist of multiple
    /// fragments (although this may only occur in single frame - need to
    /// confirm this)
    /// This function may be invoked multiple times for the same attribute
    /// due to streaming.  Handler implementations are responsible for
    /// concatenating the received data in this case
    fn pixel_data_fragment(
        &mut self,
        _attribute: &Attribute,
        _fragment_number: usize,
        _data: &[u8],
    ) -> HandlerResult {
        HandlerResult::Continue
    }
}

pub mod cancel;
pub mod data_set;
