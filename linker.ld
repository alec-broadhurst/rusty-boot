MEMORY
{
  bootloader (rx) : ORIGIN = 0x7800, LENGTH = 0x800
}

SECTIONS
{
  .text : {
    KEEP(*(.start))
    *(.text)
    *(.text.*)
    *(.rodata)
    *(.rodata.*)
  } > bootloader
}

ENTRY(start)
