# waybar-airpods

Shows the battery status of connected AirPods on [waybar](https://github.com/Alexays/Waybar).
If you hover over the module, it also shows the battery status of the individual AirPods.

![Screenshot](https://github.com/user-attachments/assets/26e77ef1-a73d-4c3d-b9f8-e0b111cf87d0)

## Installation

If you have more than one monitor, you will need multiple instances of waybar running.
But since only one application can open a connection to the AirPods over BLE at a time, this module runs as two components.
The first is a daemon that connects to the AirPods and tracks their battery status and relays it to clients over D-Bus.

To install, clone the repo, then to install the daemon, client, and systemd service just run the [`install.sh`](install.sh) script.
This requires [cargo](https://rustup.rs) to be installed.

If everything worked, running `waybar-airpods` should print a bunch of JSON messages with the status of your connected AirPods.
To display on your actual bar, update your waybar `config.jsonc` with the following module definition.

```json
"custom/airpods": {
  "exec": "waybar-airpods",
  "return-type": "json",
  "format": "{text}",
}
```

You can use the `#custom-airpods` css selector to style the module with classes of `disconnected`, `connected`, and `connected-low` (battery under 15%).

## References

- [librepods AAP Definitions](https://github.com/kavishdevar/librepods/blob/main/AAP%20Definitions.md) &mdash; Documents the general format of the AirPod control packets
