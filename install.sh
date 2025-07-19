#!/bin/bash

# Install components to /home/$USER/.cargo/bin
cargo install --path client
cargo install --path daemon

# Install systemd service
mkdir -p ~/.config/systemd/user

echo "
[Unit]
Description=Waybar Airpods Service
After=bluetooth.target

[Service]
Type=simple
Restart=on-failure
ExecStart=/home/$USER/.cargo/bin/waybar-airpods-daemon

[Install]
WantedBy=default.target
" > ~/.config/systemd/user/waybar-airpods-daemon.service

systemctl --user daemon-reload
systemctl --user enable waybar-airpods-daemon.service
systemctl --user start waybar-airpods-daemon.service
