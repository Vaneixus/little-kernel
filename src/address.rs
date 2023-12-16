
pub const PERIPHERAL_BASE: u32 = 0x3F000000; // Raspberry Pi 3 base address for all preferals.
pub const UART_DR: u32 = PERIPHERAL_BASE + 0x201000; // UART Data Registry
pub const UART_FR: u32 = PERIPHERAL_BASE + 0x201018; // UART Flag Registry