use std::mem::ManuallyDrop;

use anyhow::Result;
use common::{status::Status, waybar::Waybar};
use zbus::{Connection, conn, fdo, interface, proxy};

struct WaybarAirpodsServer {
    connection: Connection,
    clients: Vec<WaybarAirpodsClientProxy<'static>>,
}

#[proxy(
    interface = "com.connorcode.WaybarAirpodsClient",
    default_service = "com.connorcode.WaybarAirpodsClient"
)]
trait WaybarAirpodsClient {
    fn push_status(&self, waybar: &Waybar) -> Result<()>;
}

#[interface(name = "com.connorcode.WaybarAirpodsServer")]
impl WaybarAirpodsServer {
    async fn register_client(&mut self, path: String) -> fdo::Result<()> {
        let proxy = WaybarAirpodsClientProxy::builder(&self.connection)
            .path(path)?
            .build()
            .await?;

        self.clients.push(proxy);
        self.brodcast(&Waybar::from_status(&Status::default()))
            .await;
        Ok(())
    }
}

impl WaybarAirpodsServer {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            connection: Connection::session().await?,
            clients: Vec::new(),
        })
    }

    pub async fn brodcast(&self, waybar: &Waybar) {
        for client in self.clients.iter() {
            if let Err(err) = client.push_status(waybar).await {
                eprintln!("Failed to write to client: {err}");
            }
        }
    }
}

pub async fn run() -> Result<()> {
    let server = WaybarAirpodsServer::new().await?;
    let conn = conn::Builder::session()?
        .name("com.connorcode.WaybarAirpodsServer")?
        .serve_at("/com/connorcode/WaybarAirpods", server)?
        .build()
        .await?;

    println!("Started server");

    let _ = ManuallyDrop::new(conn);
    Ok(())
}
