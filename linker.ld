MEMORY
{
    bootloader   (rx)    :   ORIGIN = 0x7800,    LENGTH = 0x800
    sram    (rwx)   :   ORIGIN = 0x0100,    LENGTH = 0x800
}

SECTIONS
{
    .text : {
        *(.text)
        *(.text.*)
        *(.rodata)
        *(.rodata.*)
    } > bootloader

    .data : {
        __sram_start = .;
        *(.data)
        *(.data.*)
        __sram_end = .;
    } > sram

    .bss : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        __bss_end = .;
    } > sram
}

ENTRY(main)
