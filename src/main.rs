#![no_main]
#![no_std]
#![feature(asm_const)]

use core::arch::global_asm;
global_asm!(include_str!("boot.s"), CONST_CORE_ID_MASK = const 0b11);

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    panic!()
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
