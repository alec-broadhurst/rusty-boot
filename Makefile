defualt: clean build convert

clean:
	cargo clean

build:
	cargo build --release

convert:
	avr-objcopy -O ihex target/avr-atmega328p/release/rusty-boot.elf rusty-boot.hex

flash:
	avrdude -c arduino -p atmega328p -P /dev/tty.usbmodem1101 -b 115200 -U flash:w:rusty-boot.hex -e
