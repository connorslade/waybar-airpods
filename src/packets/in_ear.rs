use crate::consts::EAR_DETECTION;

#[derive(Debug)]
pub struct InEarPacket {
    pub primary: EarStatus,
    pub secondary: EarStatus,
}

#[derive(Debug)]
pub enum EarStatus {
    InEar = 0x00,
    NotInEar = 0x01,
    InCase = 0x02,
    Disconnected,
}

impl InEarPacket {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 8 || !bytes.starts_with(EAR_DETECTION) {
            return None;
        }

        Some(InEarPacket {
            primary: EarStatus::from(bytes[6]),
            secondary: EarStatus::from(bytes[7]),
        })
    }
}

impl EarStatus {
    pub fn from(byte: u8) -> Self {
        match byte {
            0x00 => EarStatus::InEar,
            0x01 => EarStatus::NotInEar,
            0x02 => EarStatus::InCase,
            _ => EarStatus::Disconnected,
        }
    }
}
