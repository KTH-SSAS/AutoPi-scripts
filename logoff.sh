#!/bin/bash

if [[ $(cat /etc/wpa_supplicant/wpa_supplicant.conf) == *"AutoPi-"* ]] # If there is an AutoPi network
then
    # Reset wpa_supplicant config file
    rm /etc/wpa_supplicant/wpa_supplicant.conf
    mv /etc/wpa_supplicant/wpa_supplicant.conf.bak /etc/wpa_supplicant/wpa_supplicant.conf

    # Reset wpa_supplicant so that old config is used
    pkill -f -- "wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf"
    wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf  

elif [[ $(iwconfig wlan0 2>&1) == *"AutoPi-"* ]]
then
    # Reset wpa_supplicant so that old config is used
    pkill -f -- "wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf"
    wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf  
fi
