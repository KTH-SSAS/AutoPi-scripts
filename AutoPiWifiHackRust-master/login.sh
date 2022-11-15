#!/bin/bash

#As root: append new network to wpa_supplicant.conf
NEWNETWORK="network={\n
    ssid=\"$1\"\n
    psk=\"$2\"\n
}"

# Save old config for wpa_supplicant
cp /etc/wpa_supplicant/wpa_supplicant.conf /etc/wpa_supplicant/wpa_supplicant.conf.bak

#Append new netrwork
echo -e $NEWNETWORK >> /etc/wpa_supplicant/wpa_supplicant.conf

#Restart wpa_supplicant daemon
pkill -f -- "wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf"
wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf

exit
