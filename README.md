# waybar-airpods

Shows the battery status of connected AirPods on [waybar](https://github.com/Alexays/Waybar).
If you hover over the module, it also shows the battery status of each individual AirPods.

![Screenshot](https://github.com/user-attachments/assets/3de3d40a-64d7-4084-9637-fc397358668d)

## Installation

If you have more than one monitor, you will need multiple instances of waybar running.
Because only one application can open a connection to the AirPods over BLE at a time, this module runs as two components.
The first is a daemon that connects to the AirPods and tracks their battery status and relays it to clients over dbus.

To install, first clone the repo, `git clone https://github.com/connorslade/waybar-airpods`, then to install the daemon, client, and systemd service there is the [`install.sh`](install.sh) script.
This requires [cargo](https://rustup.rs) to be installed as the project is built from source.

If everything worked, running `waybar-airpods` should print a bunch of JSON messages with the status of your connected AirPods.
To display on your actual bar, update your waybar `config.jsonc` with the following module definition.

```json
"custom/airpods": {
  "exec": "path/to/waybar-airpods",
  "return-type": "json",
  "format": "{text}",
}
```

## References

- [librepods AAP Definitions](https://github.com/kavishdevar/librepods/blob/main/AAP%20Definitions.md) &mdash; Documents the general format of the AirPod control packets
