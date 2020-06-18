/// Detects the presence of a valid DICOM P10 Header Prefix.  A valid
/// prefix consists of 132 bytes with the string "DICM" at location
/// 128.  The first 128 bytes are usually 0 but do not have to be
pub fn detect(bytes: &[u8]) -> bool {
    // check length
    if bytes.len() < 132 {
        return false;
    }

    // check for DICM
    &bytes[128..132] == b"DICM"
}

#[cfg(test)]
mod tests {
    use super::detect;

    #[test]
    fn zero_preamble_valid_prefix_returns_true() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(134, 0);
        bytes[128] = 'D' as u8;
        bytes[129] = 'I' as u8;
        bytes[130] = 'C' as u8;
        bytes[131] = 'M' as u8;

        assert_eq!(true, detect(&bytes));
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

        assert_eq!(true, detect(&bytes));
    }

    #[test]
    fn zero_preamble_invalid_prefix_returns_false() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(134, 0);

        assert_eq!(false, detect(&bytes));
    }

    #[test]
    fn short_buffer_returns_false() {
        let mut bytes: Vec<u8> = vec![];
        bytes.resize(128, 0);

        assert_eq!(false, detect(&bytes));
    }
}
