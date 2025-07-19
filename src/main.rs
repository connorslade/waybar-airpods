use anyhow::Result;
use bluer::rfcomm::{Profile, Role};
use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::{Uuid, uuid};

const AIRPODS_SERVICE: Uuid = uuid!("74ec2172-0bad-4d01-8f77-997b2be0722a");

const HANDSHAKE: &[u8] = &[0, 0, 4, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let device = get_airpods(&adapter).await?.unwrap();

    println!("Found AirPods: {:?}", device.all_properties().await?);

    // device.connect().await?;
    // device.connect_profile(&AIRPODS_SERVICE).await?;

    let profile = Profile {
        uuid: AIRPODS_SERVICE,
        role: Some(Role::Client),
        require_authentication: Some(false),
        require_authorization: Some(false),
        auto_connect: Some(false),
        ..Default::default()
    };

    let mut profile = session.register_profile(profile).await?;
    let handle = profile.next().await.unwrap();
    println!("Registered profile");

    let mut stream = handle.accept().unwrap();
    println!("Stream created");

    stream.write_all(HANDSHAKE).await.unwrap();

    loop {
        let mut buffer = vec![0; 1024];
        let bytes = stream.read(&mut buffer).await?;

        println!("Received data: {:?}", &buffer[..bytes]);
    }
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
