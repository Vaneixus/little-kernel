use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

use core::str;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::format;
use alloc::vec;

extern crate alloc;

use crate::address;

const ENTER_KEY: u8 = 0x0D;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(address::UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(address::UART_FR) & (1 << 4) > 0
}

pub fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(address::UART_DR, c as u32);
}

pub fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(address::UART_DR) as u8
}

pub fn get_text() -> String {
    let mut next_c: u8 = getc();
    let mut msg: String = String::from("");
    while next_c != ENTER_KEY {
        unsafe {
            msg = format!("{}{:?}", msg, next_c.as_ascii().unwrap_unchecked());
        }
        if next_c != ENTER_KEY {
            writec(next_c);
        }
        next_c = getc();
    };
    return msg.to_owned();
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