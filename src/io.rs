use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

use core::str;
use core::str::from_utf8;

use alloc::vec;
use alloc::vec::Vec;

use crate::address;

const ENTER_KEY: u8 = 0x0A;

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

// pub fn get_text(mut pMsg: &&str) {
//     let mut next_c: u8 = 0;
//     let mut str_in_bytes: Vec<u8> = vec![];
//     while next_c != ENTER_KEY {
//         next_c = getc();
//         str_in_bytes.append(&mut vec![next_c]);
//     };
//     pMsg = &from_utf8(&str_in_bytes).unwrap();
// }

pub fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

pub fn clear_terminal_buffer() {
    write("\x1b\x5B\x32\x4a\r"); // Clear console.
    write("\x1b[0;0H"); // return to first line.
}