use crate::value_parser::ParseError;

/// Detects the presence of a valid DICOM P10 Header Prefix.  A valid
/// prefix consists of 132 bytes with the string "DICM" at location
/// 128.  The first 128 bytes are usually 0 but do not have to be
pub fn validate(bytes: &[u8]) -> Result<(), ParseError> {
    // check length
    if bytes.len() < 132 {
        return Err(ParseError {
            reason: "must have at least 132 bytes to validate",
            position: bytes.len(),
        });
    }

    // check for DICM
    if &bytes[128..132] != b"DICM" {
        return Err(ParseError {
            reason: "DICOM not found at position 128",
            position: 128,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn zero_preamble_valid_prefix_returns_true() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(134, 0);
        bytes[128] = 'D' as u8;
        bytes[129] = 'I' as u8;
        bytes[130] = 'C' as u8;
        bytes[131] = 'M' as u8;

        let result = validate(&bytes);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn non_zero_preamble_valid_prefix_returns_true() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(134, 0);
        bytes[0] = 1;
        bytes[128] = 'D' as u8;
        bytes[129] = 'I' as u8;
        bytes[130] = 'C' as u8;
        bytes[131] = 'M' as u8;

        let result = validate(&bytes);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn zero_preamble_invalid_prefix_returns_error() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(134, 0);

        let result = validate(&bytes);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn short_buffer_returns_error() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(128, 0);

        let result = validate(&bytes);
        assert_eq!(result.is_err(), true);
    }
}
