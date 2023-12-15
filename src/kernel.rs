#![no_std]
#![feature(core_intrinsics, lang_items)]

mod boot;
mod panic;
mod io;

#[no_mangle]
pub extern fn kernel_main() {
    io::write("Hello Rust Kernel world!");

    loop {
        let c = io::getc();

        io::write("\x1b\x5B\x32\x4a\r"); // Clear console.
        io::write("\x1b[0;0H"); // return to first line.

        io::write("You have entered: ");
        io::writec(c);
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