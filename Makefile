default: clean build convert

clean:
	cargo clean

build:
	cargo build --release

convert:
	avr-objcopy -O ihex target/avr-none/release/rusty-boot.elf rusty-boot.hex

flash: clean build convert
	avrdude -c avrisp -p m328p -P /dev/cu.usbmodem12301 -b 19200 -U flash:w:rusty-boot.hex -e
