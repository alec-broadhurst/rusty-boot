MEMORY
{
    bootloader   (rx)    :   ORIGIN = 0x7800,    LENGTH = 0x800
    sram    (rwx)   :   ORIGIN = 0x0100,    LENGTH = 0x800
}

SECTIONS
{
    .vectors ORIGIN(bootloader) : {
        KEEP(*(.vectors))
    } > bootloader

    .text : {
        *(.text)
        *(.text.*)
        *(.rodata)
        *(.rodata.*)
    } > bootloader
}

ENTRY(start)
