pub unsafe fn write_page(page_address: u16, buffer_ptr: *const u8) {
    extern "C" {
        fn spm_poll();
        fn erase_page(page_address: u16);
        fn fill_page_buffer(word_dest: u16, buffer_ptr: *const u8);
        fn write_page(page_address: u16);
    }

    erase_page(page_address);
    spm_poll();
    fill_page_buffer(page_address, buffer_ptr);
    spm_poll();
    write_page(page_address);
}

pub unsafe fn reenable_rww() {
    extern "C" {
        fn reenable_rww();
    }
    reenable_rww();
}
