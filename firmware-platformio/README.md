# Display Dimmer Firmware

This is the Arduino/Platformio version of the Display Dimmer firmware.

In order to remap the I2C peripheral of the CH32V003, the underlying Arduino core was patched to add the alternate pin mappings. If the following pull request has been merged, it should be possible to remove the custom `platform_package` in `platformio.ini`:

https://github.com/openwch/arduino_core_ch32/pull/152