use std::io::{Write, stdout};

use serde::Serialize;

use crate::packets::{battery::BatteryPacket, in_ear::InEarPacket, metadata::MetadataPacket};

#[derive(Default)]
pub struct Status {
    metadata: MetadataPacket,
    battery: BatteryPacket,
    in_ear: InEarPacket,
}

#[derive(Serialize)]
struct Waybar {
    text: String,
    tooltip: Option<String>,
    class: Option<&'static str>,
    percentage: Option<f32>,
}

impl Status {
    pub fn got_packet(&mut self, data: &[u8]) {
        if let Some(metadata) = MetadataPacket::parse(&data) {
            self.metadata = metadata;
        } else if let Some(battery) = BatteryPacket::parse(&data) {
            self.battery = battery;
        } else if let Some(in_ear) = InEarPacket::parse(&data) {
            self.in_ear = in_ear;
        }

        self.print();
    }

    pub fn is_valid(&self) -> bool {
        self.metadata.is_valid() && self.battery.is_valid()
    }

    fn get_waybar(&self) -> Waybar {
        if !self.is_valid() {
            return Waybar {
                text: "󱡐".into(),
                tooltip: None,
                class: Some("disconnected"),
                percentage: None,
            };
        }

        let mut tooltip = String::new();
        for (name, status) in [
            ("Left", &self.battery.left),
            ("Right", &self.battery.right),
            ("Case", &self.battery.case),
        ] {
            if let Some(status) = status {
                tooltip.push_str(&format!("{}: {}%\n", name, status.level));
            }
        }

        Waybar {
            text: format!("{}% 󱡏", self.battery.min_pods()),
            tooltip: Some(tooltip[..tooltip.len() - 1].to_owned()),
            class: Some("connected"),
            percentage: Some(self.battery.min_pods() as f32 / 100.0),
        }
    }

    pub fn print(&self) {
        let waybar = self.get_waybar();
        let str = serde_json::to_string(&waybar).unwrap();

        let mut stdout = stdout();
        let _ = stdout.write_fmt(format_args!("{str}\n"));
        let _ = stdout.flush();
    }
}
