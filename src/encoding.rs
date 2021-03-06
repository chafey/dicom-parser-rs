use crate::vr::VR;
use std::convert::TryInto;
use std::fmt;

pub trait Encoding: fmt::Debug + Default {
    fn u16(bytes: &[u8]) -> u16;
    fn u32(bytes: &[u8]) -> u32;
    fn vr_and_length(bytes: &[u8]) -> Result<(Option<VR>, usize, usize), ()>;
}

#[derive(Default)]
pub struct ExplicitLittleEndian {}

impl fmt::Debug for ExplicitLittleEndian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExplicitLittleEndian").finish()
    }
}

impl Encoding for ExplicitLittleEndian {
    fn u16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes([bytes[0], bytes[1]].try_into().unwrap())
    }

    fn u32(bytes: &[u8]) -> u32 {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
    }

    fn vr_and_length(bytes: &[u8]) -> Result<(Option<VR>, usize, usize), ()> {
        let vr = VR::from_bytes(&bytes[4..6]);
        if VR::explicit_length_is_u32(vr) {
            if bytes.len() < 12 {
                return Err(());
            }

            Ok((
                Some(vr),
                ExplicitLittleEndian::u32(&bytes[8..12]) as usize,
                12,
            ))
        } else {
            if bytes.len() < 8 {
                return Err(());
            }
            Ok((
                Some(vr),
                ExplicitLittleEndian::u16(&bytes[6..8]) as usize,
                8,
            ))
        }
    }
}

#[derive(Default)]
pub struct ImplicitLittleEndian {}

impl fmt::Debug for ImplicitLittleEndian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImplicitLittleEndian").finish()
    }
}

impl Encoding for ImplicitLittleEndian {
    fn u16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes([bytes[0], bytes[1]].try_into().unwrap())
    }

    fn u32(bytes: &[u8]) -> u32 {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
    }

    fn vr_and_length(bytes: &[u8]) -> Result<(Option<VR>, usize, usize), ()> {
        if bytes.len() < 8 {
            return Err(());
        }

        let length = ImplicitLittleEndian::u32(&bytes[4..8]) as usize;
        Ok((None, length, 8))
    }
}

#[derive(Default)]
pub struct ExplicitBigEndian {}

impl fmt::Debug for ExplicitBigEndian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExplicitBigEndian").finish()
    }
}

impl Encoding for ExplicitBigEndian {
    fn u16(bytes: &[u8]) -> u16 {
        u16::from_be_bytes([bytes[0], bytes[1]].try_into().unwrap())
    }

    fn u32(bytes: &[u8]) -> u32 {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]].try_into().unwrap())
    }

    fn vr_and_length(bytes: &[u8]) -> Result<(Option<VR>, usize, usize), ()> {
        let vr = VR::from_bytes(&bytes[4..6]);
        if VR::explicit_length_is_u32(vr) {
            if bytes.len() < 12 {
                return Err(());
            }

            Ok((Some(vr), ExplicitBigEndian::u32(&bytes[8..12]) as usize, 12))
        } else {
            if bytes.len() < 8 {
                return Err(());
            }
            Ok((Some(vr), ExplicitBigEndian::u16(&bytes[6..8]) as usize, 8))
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::le_u32;

    #[test]
    fn read_success() {
        let bytes = vec![0xfe,0xff,0x00,0xe0];
        let value = le_u32(&bytes);
        println!("value = {}", value);
        assert_eq!(value, xfffee000);
    }
}*/
