use anyhow::Result;
use clone_macro::clone;
use futures::StreamExt;

use crate::{
    consts::{
        AIRPODS_PROFILE, AIRPODS_SERVICE, BATTERY_STATUS, EAR_DETECTION, FEATURES_ACK, HANDSHAKE,
        HANDSHAKE_ACK, METADATA, REQUEST_NOTIFICATIONS, SET_SPECIFIC_FEATURES,
    },
    packets::{battery::BatteryPacket, in_ear::InEarPacket, metadata::MetadataPacket},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod consts;
mod packets;

// airpods → 󱡏

#[tokio::main]
async fn main() -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    let mut profile = session.register_profile(AIRPODS_PROFILE.clone()).await?;

    let device = get_airpods(&adapter).await?.unwrap();
    tokio::spawn(clone!([device], async move {
        device.connect_profile(&AIRPODS_SERVICE).await.unwrap()
    }));

    while let Some(handle) = profile.next().await {
        if handle.device() != device.address() {
            continue;
        }

        let mut stream = handle.accept().unwrap();
        stream.write_all(HANDSHAKE).await.unwrap();

        loop {
            let mut data = Vec::new();

            loop {
                let mut buffer = vec![0; 1024];
                let bytes = stream.read(&mut buffer).await.unwrap();
                data.extend_from_slice(&buffer[..bytes]);

                if bytes < buffer.len() {
                    break;
                }
            }

            if data.starts_with(BATTERY_STATUS) {
                dbg!(BatteryPacket::parse(&data));
            } else if data.starts_with(METADATA) {
                dbg!(MetadataPacket::parse(&data));
            } else if data.starts_with(EAR_DETECTION) {
                dbg!(InEarPacket::parse(&data));
            } else if data.starts_with(HANDSHAKE_ACK) {
                stream.write_all(SET_SPECIFIC_FEATURES).await?;
            } else if data.starts_with(FEATURES_ACK) {
                stream.write_all(REQUEST_NOTIFICATIONS).await?;
            }
        }
    }

    Ok(())
}

async fn get_airpods(adapter: &bluer::Adapter) -> Result<Option<bluer::Device>> {
    let connected = adapter.device_addresses().await?;
    for connected in connected {
        let device = adapter.device(connected)?;
        if is_airpods(&device).await {
            return Ok(Some(device));
        }
    }

    Ok(None)
}

async fn is_airpods(device: &bluer::Device) -> bool {
    let uuids = device.uuids().await.unwrap().unwrap();
    uuids.contains(&AIRPODS_SERVICE)
}
