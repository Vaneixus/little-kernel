use crate as kernel;

#[no_mangle]
pub extern fn start() {
    kernel::kernel_main();
}