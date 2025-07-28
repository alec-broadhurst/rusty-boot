/// Writes a 128-byte page to flash memory at the given word address.
pub unsafe fn write_page(page_address: u16, buffer_ptr: *const u8) {
    extern "C" {
        fn erase_page(page_address: u16);
        fn fill_page_buffer(word_dest: u16, buffer_ptr: *const u8);
        fn write_page(page_address: u16);
    }

    erase_page(page_address);
    fill_page_buffer(page_address, buffer_ptr);
    write_page(page_address);
}

/// Re-enables the Read-While-Write (RWW) section of flash.
///
/// Should be called after the last page write to restore read access to
/// the application section of flash memory.
pub unsafe fn reenable_rww() {
    extern "C" {
        fn reenable_rww();
    }
    reenable_rww();
}
