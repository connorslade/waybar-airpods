use anyhow::Result;

mod bluetooth;
mod consts;
mod dbus;
mod packets;

#[tokio::main]
async fn main() -> Result<()> {
    dbus::run().await?;
    tokio::spawn(bluetooth::run()).await??;

    Ok(())
}
