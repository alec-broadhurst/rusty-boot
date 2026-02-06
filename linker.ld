MEMORY
{
  bootloader (rx) : ORIGIN = 0x7C00, LENGTH = 0x800
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
