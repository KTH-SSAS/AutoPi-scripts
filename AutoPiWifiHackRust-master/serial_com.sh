#!/bin/bash

#Serial communication over USB to Arduino

if [[ $(stty -F /dev/ttyACM0 cs8 9600 ignbrk -brkint -icrnl -imaxbel -opost -onlcr -isig -icanon -iexten -echo -echoe -echok -echoctl -echoke noflsh -ixon -crtscts 2>&1) == *"No such file or directory"* ]]
then
    stty -F /dev/ttyACM1 cs8 9600 ignbrk -brkint -icrnl -imaxbel -opost -onlcr -isig -icanon -iexten -echo -echoe -echok -echoctl -echoke noflsh -ixon -crtscts
    echo $1 > /dev/ttyACM1
    while [[ $(cat /dev/ttyACM1) != *"OK"*]]
    do
      echo $1 > /dev/ttyACM1
    done
fi

echo $1 > /dev/ttyACM0
while [[ $(cat /dev/ttyACM0) != *"OK"*]]
do
      echo $1 > /dev/ttyACM0
done
