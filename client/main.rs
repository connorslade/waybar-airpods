use anyhow::Result;
use bluer::rfcomm::{Profile, Role};
use clone_macro::clone;
use futures::StreamExt;

use crate::{
    consts::{
        AIRPODS_SERVICE, FEATURES_ACK, HANDSHAKE, HANDSHAKE_ACK, REQUEST_NOTIFICATIONS,
        SET_SPECIFIC_FEATURES,
    },
    status::Status,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod consts;
mod packets;
mod status;

#[tokio::main]
async fn main() -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    let profile = Profile {
        uuid: AIRPODS_SERVICE,
        role: Some(Role::Client),
        service: Some(AIRPODS_SERVICE),
        ..Default::default()
    };
    let mut profile = session.register_profile(profile).await?;

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

        let mut status = Status::default();
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

            if data.starts_with(HANDSHAKE_ACK) {
                stream.write_all(SET_SPECIFIC_FEATURES).await?;
            } else if data.starts_with(FEATURES_ACK) {
                stream.write_all(REQUEST_NOTIFICATIONS).await?;
            } else {
                status.got_packet(&data);
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
