# Rukaibox

A firmware for hitbox controllers, supporting the GRAM slim and other RP2040 based controllers with the same pinout.

[joybus-pio](https://github.com/JonnyHaystack/joybus-pio) and [haybox](https://github.com/JonnyHaystack/HayBox) were heavily referenced in the implementation.

## Goals

### Currently implemented

* Supports [GRAM slim PCB](https://gramctrl.com/products/gram-slim-pcb)
* Supports gamecube (joybus) controller protocol.
* Implementation in rust makes it easier to tweak, build and flash changes

### Things I plan to implement

* Configuration files to allow for setting up profile swap buttons, basic key remapping and SOCD setting.
* A profile for Rivals 2

### Things I would be happy for others to implement

* Profiles for other platform fighter games
* N64 support
* HID controller support
* HID keyboard support

## Non-Goals

* Support for non RP2040/pico boards
  * If boards start using the RP2350 chip, or the RP2040 is in some other way seriously outdated I will consider moving to a new chip.
* Profiles for non platform fighter games
  * Keep these in your own fork

## How to flash

1. Bring your controller's PCB into flashing (bootsel) mode. On the GRAM this is done by holding down the start button while plugging it in via USB C <-> USB A cable.
2. Download executable from latest github [Releases](https://github.com/rukai/rukaibox_firmware/releases)
3. Run downloaded flashing executable

On windows you must have winusb installed via [zadig](https://zadig.akeo.ie/), if you use a GC adapter in wii U / switch mode you have already done this.

## How to flash a custom version

1. First install [rustup](https://rustup.rs/)
2. `git clone https://github.com/rukai/rukaibox_firmware`
3. Make any changes to the firmware in `rukaibox_firmware/rukaibox_firmware`
4. Bring your controller's PCB into flashing (bootsel) mode. On the GRAM this is done by holding down the start button while plugging it in via USB C <-> USB A cable.
5. `cargo run --release -p rukaibox_flash`

On windows you must have winusb installed via [zadig](https://zadig.akeo.ie/), if you use a GC adapter in wii U / switch mode you have already done this.
