use std::collections::HashSet;

use anyhow::Result;
use btleplug::{
    api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter},
    platform::Manager,
};
use futures::StreamExt;
use uuid::{Uuid, uuid};

const AIRPODS_SERVICE: Uuid = uuid!("74ec2172-0bad-4d01-8f77-997b2be0722a");

#[tokio::main]
async fn main() -> Result<()> {
    let manager = Manager::new().await?;
    let adapter = &manager.adapters().await?[0];

    let mut events = adapter.events().await?;
    adapter.start_scan(ScanFilter::default()).await?;

    let mut connected = HashSet::new();

    while let Some(event) = events.next().await {
        // for id in connected.iter() {
        //     let peripheral = adapter.peripheral(&id).await?;
        //     println!("> {:?}", peripheral.properties().await.unwrap());
        // }

        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = adapter.peripheral(&id).await?;

                let props = peripheral.properties().await?.unwrap();
                if !props.services.contains(&AIRPODS_SERVICE) {
                    continue;
                }

                peripheral.connect().await.unwrap();
                peripheral.discover_services().await.unwrap();
                dbg!(adapter.peripheral(&id).await?.services());

                println!("Connected {id}: {:?}", props);
                connected.insert(id);
            }

            ref x @ (CentralEvent::DeviceConnected(ref id)
            | CentralEvent::DeviceUpdated(ref id)
            | CentralEvent::DeviceDisconnected(ref id)
            | CentralEvent::ManufacturerDataAdvertisement { ref id, .. }
            | CentralEvent::ServiceDataAdvertisement { ref id, .. }
            | CentralEvent::ServicesAdvertisement { ref id, .. })
                if connected.contains(id) =>
            {
                dbg!(x);
            }
            _ => {}
        }
    }

    Ok(())
}
