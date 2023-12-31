#![no_std]
#![feature(core_intrinsics, lang_items, ascii_char)]

mod boot;
mod panic;
mod io;
mod allocator;
mod address;

extern crate alloc;

#[no_mangle]
pub extern fn kernel_main() {
    allocator::init_heap();
    io::write("Hello Rust Kernel world!");

    loop {
        io::clear_terminal_buffer();
        io::write("You have entered: ");
        io::write(io::get_text().as_str());
    }
}

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target. You
// will not encounter linker errors until you use a part of
// libcore that references them, such as iterators in this program.
// In the future you may need to provide real implementations for
// these functions.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}



#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() { loop {} }