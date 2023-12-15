use crate as kernel;

#[no_mangle]
pub extern fn _start() {
    kernel::kernel_main();
}