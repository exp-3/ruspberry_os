extern crate core;

use core::intrinsics::volatile_store;

use ::PHY_IO_PERIPH_BASE;

pub enum GPFSEL {
    INPUT,
    OUTPUT,
    ALT0,
    ALT1,
    ALT2,
    ALT3,
    ALT4,
    ALT5,
}

const GPIO_BASE: *const u32 = (PHY_IO_PERIPH_BASE + 0x00200000) as *const u32;

pub fn init(pin: u32, func: GPFSEL) {
    let func_reg_offset = (pin / 10) as isize;
    let func_sel_offset = pin % 10;

    let func_reg = unsafe { GPIO_BASE.offset(func_reg_offset) as *mut u32 };

    let func_val = match func {
        GPFSEL::INPUT => 0x0,
        GPFSEL::OUTPUT => 0x1,
        GPFSEL::ALT0 => 0x4,
        GPFSEL::ALT1 => 0x5,
        GPFSEL::ALT2 => 0x6,
        GPFSEL::ALT3 => 0x7,
        GPFSEL::ALT4 => 0x3,
        GPFSEL::ALT5 => 0x2,
    };

    unsafe { volatile_store(func_reg, func_val << (func_sel_offset * 3)); }
}

pub fn write(pin: u32, value: bool) {
    let write_reg_offset = if value {
        7 + pin / 32
    } else {
        10 + pin / 32
    } as isize;
    let write_reg = unsafe { GPIO_BASE.offset(write_reg_offset) as *mut u32 };
    unsafe { volatile_store(write_reg, 1 << (pin % 32)); }
}

pub fn read(pin: u32) -> bool {
    let read_reg_offset = (13 + pin / 32) as isize;
    let read_reg = unsafe { GPIO_BASE.offset(read_reg_offset) as *const u32 };
    unsafe { *read_reg & (1 << (pin % 32)) != 0 }
}
