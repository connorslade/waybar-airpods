use anyhow::Result;
use common::waybar::Waybar;
use zbus::{conn, interface, object_server::SignalEmitter};

mod bluetooth;
mod consts;
mod packets;

struct WaybarAirpodsDaemon;

#[interface(name = "com.connorcode.WaybarAirpods")]
impl WaybarAirpodsDaemon {
    #[zbus(signal)]
    async fn waybar_update(emitter: &SignalEmitter<'_>, waybar: Waybar) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = conn::Builder::session()?
        .name("com.connorcode.WaybarAirpods")?
        .serve_at("/com/connorcode/WaybarAirpods", WaybarAirpodsDaemon)?
        .build()
        .await?;
    let interface = conn
        .object_server()
        .interface("/com/connorcode/WaybarAirpods")
        .await?;

    interface.waybar_update(Waybar::default()).await?;
    bluetooth::run(interface).await?;
    Ok(())
}
