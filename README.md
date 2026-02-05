# rusty-boot

![rusty-boot](rusty-boot.png)

A minimal bootloader for the Arduino Uno R3 written in Rust and AVR assembly.
This project implements the AVR STK500v1 protocol used by avrdude. It was built as a learning exercise in embedded Rust and makes several simplifying assumptions. It is intended only for the ATmega328P.

---

## Features

- Implemented in Rust and AVR assembly with direct hardware register access
- Fits within the 1 KB bootloader section
- Supports flashing and launching user applications via `avrdude`
- Designed specifically for Arduino Uno R3 hardware
- Uses the `avr-gcc` toolchain for linking and assembling

---

## Requirements

- Arduino Uno R3
- `avr-gcc` toolchain
- `avrdude`
- Make

> **Important:** AVR fuse bits must be configured to reserve the upper 1 KB of flash for the bootloader.

---

## Building

Build the bootloader and generate the `.hex` file:
```bash
make
```

---

## Flashing

Flash the bootloader using an external programmer and `avrdude`:
```bash
make flash
```
> **Note**: A hardware programmer is required to burn the bootloader.
