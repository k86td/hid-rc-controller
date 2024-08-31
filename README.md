
# TODO

- [x] center text in the middle of the terminal
- [x] massive cleanup when the tests are done
- [ ] vertical widgets?
- [ ] write some tests

- use https://github.com/vgasparyan/mcp4725-rs for DAL output
- use https://github.com/golemparts/rppal to interact with GPIO

# Building

Couples things had to be done to compile on the Pi (no cross-compilation yet). Had to install a couple of packages and enable eudev.

The packages I installed (apk add):
  - linux-headers
  - libudev-zero --repository="https://dl-cdn.alpinelinux.org/alpine/edge/community"
  - eudev-dev
  - eudev
  - udev-init-scripts

I also ran this script, not sure if it was needed:
  - setup-devd udev


I'll need to do a fresh install to see what *really* needs to be installed. I also really want to cross compile since it takes forever on the Pi.

On alpine linux, to enable i2c you need to change some stuff in `/etc/modules` and `/boot/config.txt`. [This](https://raspberrypi.stackexchange.com/questions/83457/can-i-enable-i2c-before-first-boot) article explains it.

The changes:
  - `/boot/config.txt` you have to add `dtparam=i2c_arm=on`.
  - `/etc/modules` you have to add `i2c-dev`.

