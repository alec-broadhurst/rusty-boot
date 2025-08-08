default: clean build convert

clean:
	cargo clean

build:
	cargo build --release

convert:
	avr-objcopy -O ihex target/avr-atmega328p/release/rusty-boot.elf rusty-boot.hex

flash: clean build convert
	avrdude -c avrisp -p atmega328p -P /dev/cu.usbmodem1101 -b 9600 -U flash:w:rusty-boot.hex -e
