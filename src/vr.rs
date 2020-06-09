#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VR {
    AE,
    AS,
    AT,
    CS,
    DA,
    DS,
    DT,
    FD,
    FL,
    IS,
    LO,
    LT,
    OB,
    OD,
    OF,
    OL,
    OW,
    OV,
    PN,
    SH,
    SL,
    SQ,
    SS,
    ST,
    SV,
    TM,
    UC,
    UI,
    UL,
    UN,
    UR,
    US,
    UT,
    UV,
    Unknown { bytes: [u8; 2] },
}

impl VR {
    pub fn from_bytes(bytes: &[u8]) -> VR {
        match bytes {
            b"AE" => VR::AE,
            b"AS" => VR::AS,
            b"AT" => VR::AT,
            b"CS" => VR::CS,
            b"DA" => VR::DA,
            b"DS" => VR::DS,
            b"DT" => VR::DT,
            b"FD" => VR::FD,
            b"FL" => VR::FL,
            b"IS" => VR::IS,
            b"LO" => VR::LO,
            b"LT" => VR::LT,
            b"OB" => VR::OB,
            b"OD" => VR::OD,
            b"OF" => VR::OF,
            b"OL" => VR::OL,
            b"OW" => VR::OW,
            b"OV" => VR::OV,
            b"PN" => VR::PN,
            b"SH" => VR::SH,
            b"SL" => VR::SL,
            b"SQ" => VR::SQ,
            b"SS" => VR::SS,
            b"ST" => VR::ST,
            b"SV" => VR::SV,
            b"TM" => VR::TM,
            b"UC" => VR::UC,
            b"UI" => VR::UI,
            b"UL" => VR::UL,
            b"UN" => VR::UN,
            b"UR" => VR::UR,
            b"US" => VR::US,
            b"UT" => VR::UT,
            b"UV" => VR::UV,
            _ => VR::Unknown {
                bytes: [bytes[0], bytes[1]],
            },
        }
    }

    pub fn explicit_length_is_u32(vr: VR) -> bool {
        match vr {
            VR::OW | VR::OB | VR::SQ | VR::OF | VR::UT | VR::UN => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VR;

    #[test]
    fn from_bytes_returns_cs() {
        let vr = VR::from_bytes(&vec![b'C', b'S']);
        assert_eq!(vr, VR::CS);
    }

    #[test]
    fn from_bytes_returns_unknown() {
        let vr = VR::from_bytes(&vec![b'X', b'X']);
        assert_eq!(
            vr,
            VR::Unknown {
                bytes: [b'X', b'X']
            }
        );
    }

    #[test]
    fn explicit_length_is_u32_returns_true() {
        assert_eq!(true, VR::explicit_length_is_u32(VR::OW));
    }
    #[test]
    fn explicit_length_is_u32_returns_false() {
        assert_eq!(false, VR::explicit_length_is_u32(VR::CS));
    }
}
