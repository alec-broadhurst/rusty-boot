use core::arch::asm;

extern "C" {
    fn erase_page(word_address: u16);
    fn write_page(page_address: u16);
    fn word_to_buf(word: u16, addr: u16);
    fn reenable_rww();
}

pub unsafe fn program_page(page_address: u16) {
    erase_page(page_address);
    write_page(page_address);
    reenable_rww();
}

pub unsafe fn write_word(word: u16, addr: u16) {
    word_to_buf(word, addr);
}

pub unsafe fn read_byte(addr: u16) -> u8 {
    let byte: u8;

    asm!(
        "lpm {b}, Z",
        b = out(reg) byte,
        in("Z") addr,
        options(nostack, preserves_flags)
    );

    byte
}
