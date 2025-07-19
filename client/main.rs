use anyhow::Result;
use futures::StreamExt;
use zbus::{Connection, proxy};

use common::waybar::Waybar;

#[proxy(
    default_service = "com.connorcode.WaybarAirpods",
    default_path = "/com/connorcode/WaybarAirpods",
    interface = "com.connorcode.WaybarAirpods"
)]
trait WaybarAirpodsDaemon {
    #[zbus(signal)]
    async fn waybar_update(&self, waybar: Waybar);
}

#[tokio::main]
async fn main() -> Result<()> {
    Waybar::not_connected().print();

    let connection = Connection::session().await?;
    let proxy = WaybarAirpodsDaemonProxy::new(&connection).await?;

    let mut stream = proxy.receive_waybar_update().await?;
    while let Some(msg) = stream.next().await {
        let waybar = msg.args()?.waybar;
        waybar.print();
    }

    Ok(())
}
