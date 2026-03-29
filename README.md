#  MASK Split keyboard Firmware

This firmware is built using [rmk](https://github.com/haobogu/rmk).

## Build firmware

You can build firmware for central and peripheral separately:

```shell
# Build central firmware
cargo build --release --bin central

# Build peripheral firmware
cargo build --release --bin peripheral

# Compile RMK and get .uf2:
cargo make uf2 --release
```

## With debugging probe

With a debugging probe, you can have the full control of you hardware. To use RMK you should check whether the bootloader is flashed to your board first. To use RMK with existing bootloader such as [Adafruit_nRF52_Bootloader](https://github.com/adafruit/Adafruit_nRF52_Bootloader), check `memory.x` in the project root, ensure that the flash starts from 0x00001000

```
MEMORY
{
  /* NOTE 1 K = 1 KiB = 1024 bytes */
  /* These values correspond to the nRF52840 WITH Adafruit nRF52 bootloader */
  FLASH : ORIGIN = 0x00001000, LENGTH = 1020K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K
}
```

Or you can use RMK without bootloader:

```
MEMORY
{
  /* NOTE 1 K = 1 KiB = 1024 bytes */
  /* These values correspond to the nRF52840 */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
```

After you have `memory.x` set, use `cargo run --release` to flash the RMK firmware to your board:

1. Enter example folder:
   ```shell
   cd examples/use_rust/nrf52840_ble_split
   ```
2. Compile, flash and run the example
   ```shell
   # Run central firmware
   cargo run --release --bin central

   # Run peripheral firmware
   cargo run --release --bin peripheral
   ```
