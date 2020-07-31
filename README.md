# arrled
Arduino FastLED based serial controller. (Because I don't wanna buy into MSI's led controller stuff)

## Usage
- Edit `src/config.hpp` to your liking. (and led strip!)
- Use PlatformIO to build and upload the firmware to your Arduino Nano. (or other compatible microcontroller)
- Edit `driver/src/config.rs` to your liking.
- Run the rust driver with the serial device path. (e.g. `cargo run /dev/ttyUSB0`, you can edit the code if you want different effects)
- Profit.