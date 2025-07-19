use std::io::{Write, stdout};

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

use crate::status::Status;

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
        for (name, status) in [
            ("Left", &status.left),
            ("Right", &status.right),
            ("Case", &status.case),
        ] {
            if let Some(status) = status {
                tooltip.push_str(&format!("{}: {}%\n", name, status));
            }
        }

        let min_pods = status.min_pods();
        let class = ["connected", "connected-low"][(min_pods <= 15) as usize];

        Waybar {
            text: format!("{min_pods}% 󱡏"),
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
