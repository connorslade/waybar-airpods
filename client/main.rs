use std::future::pending;

use anyhow::Result;
use uuid::Uuid;
use zbus::{conn, interface};

use common::waybar::Waybar;

struct WaybarAirpodsClient;

#[interface(name = "com.connorcode.WaybarAirpodsClient")]
impl WaybarAirpodsClient {
    fn push_status(&self, waybar: Waybar) {
        waybar.print();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let uuid = Uuid::new_v4();
    let path = format!("/com/connorcode/WaybarAirpods/{}", uuid.as_u128());

    let _conn = conn::Builder::session()?
        .name("com.connorcode.WaybarAirpodsClient")?
        .serve_at(path, WaybarAirpodsClient)?
        .build()
        .await?;

    pending::<()>().await;
    Ok(())
}
