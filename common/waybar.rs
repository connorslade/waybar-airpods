use std::io::{Write, stdout};

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

use crate::status::{BatteryStatus, Components, Status};

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct Waybar {
    text: String,
    tooltip: Option<String>,
    class: Option<String>,
    percentage: Option<f32>,
}

impl Waybar {
    pub fn not_connected() -> Self {
        Waybar {
            text: "󰟦".into(),
            tooltip: Some("Daemon not active".into()),
            class: Some("disconnected".into()),
            percentage: None,
        }
    }

    pub fn from_status(status: &Status) -> Self {
        if !status.is_valid() {
            return Waybar::default();
        }

        let mut tooltip = String::new();
        if let Some(metadata) = &status.metadata {
            tooltip.push_str(&format!("{} ({})\n", metadata.name, metadata.model));
        }

        let Components { left, right, case } = &status.components;
        for (idx, (name, component)) in [("Left", left), ("Right", right), ("Case", case)]
            .iter()
            .enumerate()
        {
            let Some(component) = component else {
                continue;
            };

            let icon = match component.status {
                BatteryStatus::Charging => "󰢝",
                BatteryStatus::Discharging => match idx {
                    0 => status.ear.left,
                    1 => status.ear.right,
                    _ => crate::status::EarStatus::Disconnected,
                }
                .icon(),
                BatteryStatus::Disconnected => continue,
            };

            tooltip.push_str(&format!("{icon} {name}: {}%\n", component.level));
        }

        let min_pods = status.min_pods();
        let is_low = min_pods <= 15;

        let class = ["connected", "connected-low"][is_low as usize];
        let battery = ["", "󱃍"][is_low as usize];

        let text = if min_pods == u8::MAX {
            format!("󱡏{battery}")
        } else {
            format!("{min_pods}% 󱡏{battery}")
        };

        Waybar {
            text,
            tooltip: Some(tooltip[..tooltip.len() - 1].to_owned()),
            class: Some(class.into()),
            percentage: Some(min_pods as f32 / 100.0),
        }
    }

    pub fn print(&self) {
        let str = serde_json::to_string(&self).unwrap();

        let mut stdout = stdout();
        let _ = stdout.write_fmt(format_args!("{str}\n"));
        let _ = stdout.flush();
    }
}

impl Default for Waybar {
    fn default() -> Self {
        Waybar {
            text: "󱡐".into(),
            tooltip: None,
            class: Some("disconnected".into()),
            percentage: None,
        }
    }
}
