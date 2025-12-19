MEMORY
{
  bootloader (rx) : ORIGIN = 0x7800, LENGTH = 0x800
  sram (rwx) : ORIGIN = 0x0100, LENGTH = 0x800
}

SECTIONS
{
  .text : {
    KEEP(*(.vectors))
    *(.text)
    *(.text.*)
    *(.rodata)
    *(.rodata.*)
  } > bootloader

  .data : {
    __data_start = .;
    *(.data)
    *(.data.*)
    __data_end = .;
  } > sram AT > bootloader

  .bss : {
    __bss_start = .;
    *(.bss)
    *(.bss.*)
    __bss_end = .;
  } > sram

  __data_load_start = LOADADDR(.data);
  __data_load_end = __data_load_start + SIZEOF(.data);
}

ENTRY(start)
