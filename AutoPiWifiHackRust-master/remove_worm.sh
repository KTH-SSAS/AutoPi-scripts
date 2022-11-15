#!/bin/bash

# Run with sudo setsid
# Helper script to remove worm and reset AutoPi

killall dumpexec.sh
killall rostigare
killall login.sh
killall start.sh

rm -r /home/pi/hack_folder

umount /home/pi/list
rm -r /home/pi/list

rm /usr/bin/sshpass

rm /usr/local/bin/start.sh
rm /usr/local/bin/dumpexec.sh
rm /usr/local/bin/login.sh

echo "192.168.4.1 local.autopi.io" > /etc/dnsmasq.hosts
systemctl restart dnsmasq
echo -e "hostname\nclientid\npersistent\noption rapid_commit\noption domain_name_servers, domain_name, domain_search, host_name\noption classless_static_routes\noption ntp_servers\noption interface_mtu\nrequire dhcp_server_identifier\nslaac-private\n\ninterface wlan0\nmetric 200\nnohook wpa_supplicant\n\ninterface uap0\nstatic ip_address=192.168.4.1/24\nmetric 100\nnohook wpa_supplicant\n" > /etc/dhcpcd.conf
systemctl restart dhcpcd

ip addr del 192.168.4.100/24 dev uap0
 
rm /home/pi/start.sh
rm /home/pi/dumpexec.sh
rm /home/pi/login.sh
rm /home/pi/logoff.sh
rm /home/pi/rostigare
rm /home/pi/hacked_networks
rm /home/pi/remove_worm.sh
rm -r /home/pi/sshpass-pkg
rm /home/pi/*.deb

echo -e 'ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev\nupdate_config=1\ncountry=GB\nnetwork={\npriority=1\npsk="qwer1234"\nssid="AndroidAP4F86"\n}' > /etc/wpa_supplicant/wpa_supplicant.conf
pkill -f -- "wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf"
wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf
