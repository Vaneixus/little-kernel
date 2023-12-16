use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
use core::str;

const PERIPHERAL_BASE: u32 = 0x3F000000;
const UART_DR: u32 = PERIPHERAL_BASE + 0x201000;
const UART_FR: u32 = PERIPHERAL_BASE + 0x201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

pub fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

pub fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

pub fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

pub fn clear_terminal_buffer() {
    write("\x1b\x5B\x32\x4a\r"); // Clear console.
    write("\x1b[0;0H"); // return to first line.
}