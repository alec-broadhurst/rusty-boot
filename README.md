# rusty-boot

A bootloader for the Arduino Uno R3 written in Rust and Assembly.
This project targets the ATmega328P microcontroller and implements a minimal UART-based flashing protocol.

---

## Features

- Written in Rust and AVR Assembly with direct hardware register access
- Occupies a 2 KB bootloader section
- Waits briefly for serial input, then jumps to the main application if none is received
- Specifically designed for the Arduino Uno R3 (ATmega328P)

> **Note**: You must configure AVR fuse bits to reserve the upper 2 KB of flash for the bootloader.

---

## Building

Compile the project using either `make` or `cargo`:

```bash
make
```
or
```bash
cargo build --release
```

Then convert the ELF output to Intel HEX format for flashing:
```bash
avr-objcopy -O ihex target/avr-atmega328p/release/rusty-boot.elf rusty-boot.hex
```
