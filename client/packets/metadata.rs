use crate::consts::METADATA;

#[derive(Default, Debug)]
pub struct MetadataPacket {
    pub device_name: String,
    pub model_number: String,
    pub manufacturer: String,
}

impl MetadataPacket {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 11 || !data.starts_with(METADATA) {
            return None;
        }

        let data = &data[11..];
        let mut start = 0;
        let mut idx = 0;

        let mut read_string = || {
            start = idx;
            while idx < data.len() && data[idx] != 0x00 {
                idx += 1;
            }

            let out = String::from_utf8_lossy(&data[start..idx]).to_string();
            idx += 1;

            out
        };

        Some(Self {
            device_name: read_string(),
            model_number: read_string(),
            manufacturer: read_string(),
        })
    }

    pub fn is_valid(&self) -> bool {
        !self.device_name.is_empty()
            && !self.model_number.is_empty()
            && !self.manufacturer.is_empty()
    }
}
