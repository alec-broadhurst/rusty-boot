MEMORY
{
    flash   (rx)    :   ORIGIN = 0x0000,    LENGTH = 30K
    sram    (rwx)   :   ORIGIN = 0x0800,    LENGTH = 2k
}

SECTIONS
{
    .text : {
        *(.text)
    } > flash

    .data : {
        __sram_start = .;
        *(.data)
        __sram_end = .;
    } > sram

    .bss : {
        __bss_start = .;
        *(.bss)
        __bss_end = .;
    } > sram
}

__sram_load_start = ADDR(.data);

ENTRY(main)
