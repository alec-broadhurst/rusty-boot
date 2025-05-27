pub struct Flash;

impl Flash {
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
}
