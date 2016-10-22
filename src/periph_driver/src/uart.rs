extern crate core;
use core::intrinsics::volatile_store;

use ::PHY_IO_PERIPH_BASE;

const UART0_BASE: *const u32 = (PHY_IO_PERIPH_BASE + 0x00201000) as *const u32;

pub fn init() {
    let ibrd = unsafe {UART0_BASE.offset(9) as *mut u32};
    let fbrd = unsafe {UART0_BASE.offset(10) as *mut u32};
    let lcrh = unsafe {UART0_BASE.offset(11) as *mut u32};
    let cr = unsafe {UART0_BASE.offset(12) as *mut u32};

    unsafe { volatile_store(cr, 0); }

    // TODO: set pin mode

    unsafe {
        // set baudrate 115200
        // (3000000 / (16 * 115200)) = 1.627
        // so, ibrd = 1
        // and, fbrd = (0.627 * 64) + 0.5 = 40
        volatile_store(ibrd, 1);
        volatile_store(fbrd, 40);

        // stick parity disable, 8bit, FIFO enable,
        // two stop bit no, odd parity,
        // parity disable, break no
        volatile_store(lcrh, 0x70);

        // CTS disable, RTS disable, OUT1 - 2 = 0,
        // DTR disable, RXE enable, TXE enable,
        // loop back disable, SIRLP=0, STREN = 0,
        // UARTEN enable
        volatile_store(cr, 0x0301);
    }
}

pub fn read_byte()->u8 {
    let dr = UART0_BASE as *const u32;
    let fr = unsafe {UART0_BASE.offset(6) as *const u32};
    unsafe {
        while (*fr & (1 << 4)) != 0 {
            asm!("");
        }

        (0xff & *dr) as u8
    }
}

pub fn write_byte(c: u8) {
    let dr = UART0_BASE as *mut u32;
    let fr = unsafe {UART0_BASE.offset(6) as *const u32};
    unsafe {
        while (*fr & (1 << 5)) != 0 {
            asm!("");
        }

        volatile_store(dr, (0xff & c) as u32);
    }
}
