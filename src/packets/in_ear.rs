use crate::consts::EAR_DETECTION;

#[derive(Debug)]
pub struct InEarPacket {
    pub primary: InEar,
    pub secondary: InEar,
}

#[derive(Debug)]
pub enum InEar {
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
            primary: InEar::from(bytes[6]),
            secondary: InEar::from(bytes[7]),
        })
    }
}

impl InEar {
    pub fn from(byte: u8) -> Self {
        match byte {
            0x00 => InEar::InEar,
            0x01 => InEar::NotInEar,
            0x02 => InEar::InCase,
            _ => InEar::Disconnected,
        }
    }
}
