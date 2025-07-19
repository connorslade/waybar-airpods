use std::time::Duration;

use anyhow::Result;
use bluer::{
    DeviceEvent, DeviceProperty, ErrorKind,
    rfcomm::{Profile, Role, Stream},
};
use futures::StreamExt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time,
};
use zbus::object_server::InterfaceRef;

use crate::{
    WaybarAirpodsDaemon, WaybarAirpodsDaemonSignals,
    consts::{
        AIRPODS_SERVICE, FEATURES_ACK, HANDSHAKE, HANDSHAKE_ACK, REQUEST_NOTIFICATIONS,
        SET_SPECIFIC_FEATURES,
    },
    packets::{
        battery::{BatteryPacket, Pod},
        in_ear::InEarPacket,
        metadata::MetadataPacket,
    },
};
use common::{status::Status, waybar::Waybar};

#[derive(Default)]
struct State {
    primary: Pod,
    status: Status,
}

pub async fn run(interface: InterfaceRef<WaybarAirpodsDaemon>) -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    let profile = Profile {
        uuid: AIRPODS_SERVICE,
        role: Some(Role::Client),
        service: Some(AIRPODS_SERVICE),
        ..Default::default()
    };
    let mut profile = session.register_profile(profile).await?;

    let device = get_airpods(&adapter).await?;
    println!("Found Airpods [{}]", device.address());

    tokio::spawn(async move {
        if device.is_connected().await.unwrap() {
            device.connect_profile(&AIRPODS_SERVICE).await.unwrap();
        }

        let mut events = device.events().await.unwrap();
        while let Some(event) = events.next().await {
            let DeviceEvent::PropertyChanged(DeviceProperty::Connected(true)) = event else {
                continue;
            };

            while let Err(err) = device.connect_profile(&AIRPODS_SERVICE).await
                && err.kind != ErrorKind::InProgress
            {}
        }
    });

    while let Some(handle) = profile.next().await {
        let mut stream = handle.accept().unwrap();
        if let Err(err) = handle_connection(&interface, &mut stream).await {
            interface.waybar_update(Waybar::default()).await?;
            println!("Disconnected: {err}");
        }
    }

    Ok(())
}

async fn handle_connection(
    interface: &InterfaceRef<WaybarAirpodsDaemon>,
    stream: &mut Stream,
) -> Result<()> {
    stream.write_all(HANDSHAKE).await.unwrap();

    let mut state = State::default();
    loop {
        let mut data = Vec::new();

        loop {
            let mut buffer = vec![0; 1024];
            let bytes = stream.read(&mut buffer).await?;
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
            let hash = state.status.hash();
            got_packet(&mut state, &data);

            if hash != state.status.hash() {
                let waybar = Waybar::from_status(&state.status);
                interface.waybar_update(waybar).await?;
            }
        }
    }
}

fn got_packet(state: &mut State, data: &[u8]) {
    if let Some(metadata) = MetadataPacket::parse(data) {
        println!("{metadata:?}");
        state.status.metadata = Some(metadata.into());
    } else if let Some(battery) = BatteryPacket::parse(data) {
        println!("{battery:?}");
        state.primary = battery.primary;

        let mut status_comp = state.status.components.as_arr_mut();
        let battery_comp = battery.as_arr();
        for (status, battery) in status_comp.iter_mut().zip(battery_comp) {
            if let Some(battery) = *battery {
                **status = Some(battery.into());
            }
        }
    } else if let Some(in_ear) = InEarPacket::parse(data) {
        println!("{in_ear:?}");

        let Some([left, right]) = in_ear.get(state.primary) else {
            return;
        };

        state.status.ear.left = left.into();
        state.status.ear.right = right.into();
    }
}

async fn get_airpods(adapter: &bluer::Adapter) -> Result<bluer::Device> {
    loop {
        let connected = adapter.device_addresses().await?;
        for connected in connected {
            let device = adapter.device(connected)?;

            let uuids = device.uuids().await.unwrap().unwrap();
            if uuids.contains(&AIRPODS_SERVICE) {
                return Ok(device);
            }
        }

        time::sleep(Duration::from_secs(15)).await;
    }
}
